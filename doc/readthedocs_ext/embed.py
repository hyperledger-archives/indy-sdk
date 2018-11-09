from __future__ import print_function

from docutils import nodes
from docutils.parsers.rst import Directive

import requests


def get_inline_html(api_host, project, version, doc, section):
    url = '{api_host}/api/v2/embed/'.format(api_host=api_host)
    resp = requests.get(
        url,
        params={'project': project, 'version': version, 'doc': doc, 'section': section}
    )
    return resp.json()['wrapped'][0]


class readthedocsembed(nodes.raw):

    """
    Holding class for our embeds
    """
    pass


class EmbedDirective(Directive):

    """
    The main directive for readthedocs-embed

    Allows users to embed HTML documentation from any Read the Docs project.
    """

    # this enables content in the directive
    has_content = True
    option_spec = {
        'project': str,
        'version': str,
        'doc': str,
        'section': str,
    }

    def run(self):
        env = self.state.document.settings.env
        api_host = self.app.builder.config.html_context.get('api_host', 'https://readthedocs.org')

        project = self.options.get('project', env.config.readthedocs_embed_project)
        version = self.options.get('version', env.config.readthedocs_embed_version)
        doc = self.options.get('doc', env.config.readthedocs_embed_doc)
        section = self.options.get('section')
        if not (project and version and doc and section):
            return [self.state.document.reporter.error(
                '[readthedocs-embed] Requires project, version, doc, and section',
                line=self.lineno
            )]

        try:
            inline_content = get_inline_html(api_host, project, version, doc, section)
            node = readthedocsembed('', inline_content, format='html')
        except Exception as e:
            return [self.state.document.reporter.error(
                '[readthedocs-embed] Fetching embed HTML failed: %s' % e.msg, line=self.lineno)]
        return [node]


def setup(app):
    """
    This isn't used in Production,
    but allows this module to be used as a standalone extension.
    """

    app.add_directive('readthedocs-embed', EmbedDirective)
    app.add_config_value('readthedocs_embed_project', '', 'html')
    app.add_config_value('readthedocs_embed_version', '', 'html')
    app.add_config_value('readthedocs_embed_doc', '', 'html')

    return app
