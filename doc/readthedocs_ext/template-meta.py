"""
Allow a project to specify the specific template to render in the page context.

This works by putting this at the top of any RST page::

    :template: path/to/template.html

You can add additional templates based on the ``templates_path`` setting.
"""


def on_page_context(app, pagename, templatename, context, doctree):
    if 'meta' in context and 'template' in context['meta']:
        return context['meta']['template']

    return None


def setup(app):
    app.connect('html-page-context', on_page_context)
