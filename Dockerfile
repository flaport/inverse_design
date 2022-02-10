FROM condaforge/mambaforge
ADD environment.yml .
RUN conda env update -n base -f environment.yml
