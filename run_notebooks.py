import os
import papermill

ROOT = os.path.dirname(os.path.abspath(__file__))

def iter_notebooks():
    for root, _, fns in os.walk(ROOT):
        if "_proc" in root:
            continue
        if ".ipynb_checkpoints" in root:
            continue
        for fn in fns:
            if not fn.endswith('.ipynb'):
                continue
            yield os.path.join(root, fn)

for i, path in enumerate(iter_notebooks()):
    if i > 9: # don't run unfinished notebooks
        break
    print(path)
    cwd = os.path.dirname(path)
    try:
        papermill.execute_notebook(
            input_path=path,
            output_path=path,
            cwd=cwd,
        )
    except Exception:
        print("failed.")
