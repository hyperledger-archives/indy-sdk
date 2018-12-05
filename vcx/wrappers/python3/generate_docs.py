import os
import pydoc
import sys

class DocTree:
    def __init__(self, src, dest):
        self.basepath = os.getcwd()
        sys.path.append(os.path.join(self.basepath, src))
        self.src = src
        self.dest = dest
        self._make_dest(dest)
        self._make_docs(src)
        self._move_docs(dest)

    def _make_dest(self, dest):
        path = os.path.join(self.basepath, dest)
        if os.path.isdir(path):
            os.rmdir(path)
        os.makedirs(path)

    def _make_docs(self, src):
        print('making htmls for ' + src)
        pydoc.writedocs(src)
        print(os.listdir())

    def _move_docs(self, dest):

        for f in os.listdir():
            if f.endswith('.html'):
                _dest = os.path.join(dest, f)
                os.rename(f, _dest)


def main():
    dest = 'docs'
    src = 'vcx/api'
    src = os.path.join(os.getcwd(), src)
    ft = DocTree(src, dest)

if __name__ == '__main__':
    main()

