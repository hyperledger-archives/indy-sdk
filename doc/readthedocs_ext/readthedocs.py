# -*- coding: utf-8 -*-

from __future__ import absolute_import

import codecs
import json
import os
import re
import types
from distutils.version import LooseVersion

import sphinx
from sphinx import package_dir
from sphinx.builders.html import (DirectoryHTMLBuilder, SingleFileHTMLBuilder,
                                  StandaloneHTMLBuilder)
from sphinx.util.console import bold

from .embed import EmbedDirective
from .mixins import BuilderMixin

try:
    # Avaliable from Sphinx 1.6
    from sphinx.util.logging import getLogger
except ImportError:
    from logging import getLogger

log = getLogger(__name__)

MEDIA_MAPPING = {
    "_static/jquery.js": "%sjavascript/jquery/jquery-2.0.3.min.js",
    "_static/underscore.js": "%sjavascript/underscore.js",
    "_static/doctools.js": "%sjavascript/doctools.js",
}


# Whitelist keys that we want to output
# to the json artifacts.
KEYS = [
    'body',
    'title',
    'sourcename',
    'current_page_name',
    'toc',
    'page_source_suffix',
]


def finalize_media(app):
    """ Point media files at our media server. """

    if (app.builder.name == 'readthedocssinglehtmllocalmedia' or
            app.builder.format != 'html' or
            not hasattr(app.builder, 'script_files')):
        return  # Use local media for downloadable files
    # Pull project data from conf.py if it exists
    context = app.builder.config.html_context
    MEDIA_URL = context.get('MEDIA_URL', 'https://media.readthedocs.org/')

    # Put in our media files instead of putting them in the docs.
    for index, file in enumerate(app.builder.script_files):
        if file in MEDIA_MAPPING.keys():
            app.builder.script_files[index] = MEDIA_MAPPING[file] % MEDIA_URL
            if file == "_static/jquery.js":
                app.builder.script_files.insert(
                    index + 1, "%sjavascript/jquery/jquery-migrate-1.2.1.min.js" % MEDIA_URL)

    app.builder.script_files.append(
        '%sjavascript/readthedocs-doc-embed.js' % MEDIA_URL
    )


def update_body(app, pagename, templatename, context, doctree):
    """
    Add Read the Docs content to Sphinx body content.

    This is the most reliable way to inject our content into the page.
    """

    MEDIA_URL = context.get('MEDIA_URL', 'https://media.readthedocs.org/')
    if app.builder.name == 'readthedocssinglehtmllocalmedia':
        if 'html_theme' in context and context['html_theme'] == 'sphinx_rtd_theme':
            theme_css = '_static/css/theme.css'
        else:
            theme_css = '_static/css/badge_only.css'
    elif app.builder.name in ['readthedocs', 'readthedocsdirhtml']:
        if 'html_theme' in context and context['html_theme'] == 'sphinx_rtd_theme':
            theme_css = '%scss/sphinx_rtd_theme.css' % MEDIA_URL
        else:
            theme_css = '%scss/badge_only.css' % MEDIA_URL
    else:
        # Only insert on our HTML builds
        return

    inject_css = True

    # Starting at v0.4.0 of the sphinx theme, the theme CSS should not be injected
    # This decouples the theme CSS (which is versioned independently) from readthedocs.org
    if theme_css.endswith('sphinx_rtd_theme.css'):
        try:
            import sphinx_rtd_theme
            inject_css = LooseVersion(sphinx_rtd_theme.__version__) < LooseVersion('0.4.0')
        except ImportError:
            pass

    if inject_css and theme_css not in app.builder.css_files:
        app.builder.css_files.insert(0, theme_css)

    # This is monkey patched on the signal because we can't know what the user
    # has done with their `app.builder.templates` before now.

    if not hasattr(app.builder.templates.render, '_patched'):
        # Janky monkey patch of template rendering to add our content
        old_render = app.builder.templates.render

        def rtd_render(self, template, render_context):
            """
            A decorator that renders the content with the users template renderer,
            then adds the Read the Docs HTML content at the end of body.
            """
            # Render Read the Docs content
            template_context = render_context.copy()
            template_context['rtd_css_url'] = '{}css/readthedocs-doc-embed.css'.format(MEDIA_URL)
            template_context['rtd_analytics_url'] = '{}javascript/readthedocs-analytics.js'.format(
                MEDIA_URL,
            )
            source = os.path.join(
                os.path.abspath(os.path.dirname(__file__)),
                '_templates',
                'readthedocs-insert.html.tmpl'
            )
            templ = open(source).read()
            rtd_content = app.builder.templates.render_string(templ, template_context)

            # Handle original render function
            content = old_render(template, render_context)
            end_body = content.lower().find('</head>')

            # Insert our content at the end of the body.
            if end_body != -1:
                content = content[:end_body] + rtd_content + "\n" + content[end_body:]
            else:
                log.debug("File doesn't look like HTML. Skipping RTD content addition")

            return content

        rtd_render._patched = True
        app.builder.templates.render = types.MethodType(rtd_render,
                                                        app.builder.templates)


def generate_json_artifacts(app, pagename, templatename, context, doctree):
    """
    Generate JSON artifacts for each page.

    This way we can skip generating this in other build step.
    """
    try:
        # We need to get the output directory where the docs are built
        # _build/json.
        build_json = os.path.abspath(
            os.path.join(app.outdir, '..', 'json')
        )
        outjson = os.path.join(build_json, pagename + '.fjson')
        outdir = os.path.dirname(outjson)
        if not os.path.exists(outdir):
            os.makedirs(outdir)
        with open(outjson, 'w+') as json_file:
            to_context = {
                key: context.get(key, '')
                for key in KEYS
            }
            json.dump(to_context, json_file, indent=4)
    except TypeError:
        log.exception(
            'Fail to encode JSON for page {page}'.format(page=outjson)
        )
    except IOError:
        log.exception(
            'Fail to save JSON output for page {page}'.format(page=outjson)
        )
    except Exception as e:
        log.exception(
            'Failure in JSON search dump for page {page}'.format(page=outjson)
        )


class HtmlBuilderMixin(BuilderMixin):

    static_readthedocs_files = [
        'readthedocs-data.js_t',
        # We patch searchtools and copy it with a special handler
        # 'searchtools.js_t'
    ]

    REPLACEMENT_TEXT = '/* Search initialization removed for Read the Docs */'
    REPLACEMENT_PATTERN = re.compile(
        r'''
        ^\$\(document\).ready\(function\s*\(\)\s*{(?:\n|\r\n?)
        \s*Search.init\(\);(?:\n|\r\n?)
        \}\);
        ''',
        (re.MULTILINE | re.VERBOSE)
    )

    def get_static_readthedocs_context(self):
        ctx = super(HtmlBuilderMixin, self).get_static_readthedocs_context()
        if self.indexer is not None:
            ctx.update(self.indexer.context_for_searchtool())
        return ctx

    def copy_static_readthedocs_files(self):
        super(HtmlBuilderMixin, self).copy_static_readthedocs_files()
        self._copy_searchtools()

    def _copy_searchtools(self, renderer=None):
        """Copy and patch searchtools

        This uses the included Sphinx version's searchtools, but patches it to
        remove automatic initialization. This is a fork of
        ``sphinx.util.fileutil.copy_asset``
        """
        log.info(bold('copying searchtools... '), nonl=True)

        path_src = os.path.join(package_dir, 'themes', 'basic', 'static',
                                'searchtools.js_t')
        if os.path.exists(path_src):
            path_dest = os.path.join(self.outdir, '_static', 'searchtools.js')
            if renderer is None:
                # Sphinx 1.4 used the renderer from the existing builder, but
                # the pattern for Sphinx 1.5 is to pass in a renderer separate
                # from the builder. This supports both patterns for future
                # compatibility
                if sphinx.version_info < (1, 5):
                    renderer = self.templates
                else:
                    from sphinx.util.template import SphinxRenderer
                    renderer = SphinxRenderer()
            with codecs.open(path_src, 'r', encoding='utf-8') as h_src:
                with codecs.open(path_dest, 'w', encoding='utf-8') as h_dest:
                    data = h_src.read()
                    data = self.REPLACEMENT_PATTERN.sub(self.REPLACEMENT_TEXT, data)
                    h_dest.write(renderer.render_string(
                        data,
                        self.get_static_readthedocs_context()
                    ))
        else:
            log.warning('Missing searchtools.js_t')
        log.info('done')


class ReadtheDocsBuilder(HtmlBuilderMixin, StandaloneHTMLBuilder):
    name = 'readthedocs'


class ReadtheDocsDirectoryHTMLBuilder(HtmlBuilderMixin, DirectoryHTMLBuilder):
    name = 'readthedocsdirhtml'


class ReadtheDocsSingleFileHTMLBuilder(BuilderMixin, SingleFileHTMLBuilder):
    name = 'readthedocssinglehtml'


class ReadtheDocsSingleFileHTMLBuilderLocalMedia(BuilderMixin, SingleFileHTMLBuilder):
    name = 'readthedocssinglehtmllocalmedia'


def setup(app):
    app.add_builder(ReadtheDocsBuilder)
    app.add_builder(ReadtheDocsDirectoryHTMLBuilder)
    app.add_builder(ReadtheDocsSingleFileHTMLBuilder)
    app.add_builder(ReadtheDocsSingleFileHTMLBuilderLocalMedia)
    app.connect('builder-inited', finalize_media)
    app.connect('html-page-context', update_body)
    app.connect('html-page-context', generate_json_artifacts)

    # Embed
    app.add_directive('readthedocs-embed', EmbedDirective)
    app.add_config_value('readthedocs_embed_project', '', 'html')
    app.add_config_value('readthedocs_embed_version', '', 'html')
    app.add_config_value('readthedocs_embed_doc', '', 'html')
    app.add_config_value('rtd_generate_json_artifacts', False, 'html')

    return {}
