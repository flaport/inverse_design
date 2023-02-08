import os
import sys
import time
import papermill
from fastcore.parallel import parallel
import random

ROOT = os.path.dirname(os.path.abspath(__file__))


def iter_notebooks():
    for root, _, fns in os.walk(ROOT):
        if "_proc" in root:
            continue
        if ".ipynb_checkpoints" in root:
            continue
        for fn in fns:
            if not fn.endswith(".ipynb"):
                continue
            yield os.path.join(root, fn)


def get_notebooks(skip=None):
    if skip is None:
        skip = []
    notebooks = []
    for path in iter_notebooks():
        fn = path.replace("\\", "/").split("/")[-1]
        if fn in skip:
            continue
        notebooks.append(path)
    return sorted(notebooks)


def run_notebook(path):
    fn = path.replace("\\", "/").split("/")[-1]
    print(f"START {fn}")
    cwd = os.path.dirname(path)

    sys.stdout, old_stdout = open(os.devnull, "w"), sys.stdout
    sys.stderr, old_stderr = sys.stdout, sys.stderr
    start_time = time.time()
    status = "SUCCESS"
    try:
        papermill.execute_notebook(
            input_path=path,
            output_path=path,
            cwd=cwd,
            progress_bar=False,
        )
    except Exception:
        status = "FAIL"
    finally:
        time_spent = time.time() - start_time
        sys.stdout = old_stdout
        sys.stderr = old_stderr

    print(f"{status} {fn} [time: {time_spent:.0f}s]")


if __name__ == '__main__':
    notebooks = get_notebooks(
        #skip=["10_inverse_design_local.ipynb", "11_ceviche_challenges.ipynb"]
        skip=[]
    )
    parallel(run_notebook, notebooks)
