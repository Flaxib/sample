FROM python:3.10-slim

RUN apt update && apt install --no-install-recommends -y gcc libc6-dev postgresql-client-13 postgresql-13 emacs curl \
    && apt autoremove -y \
    && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /back/
COPY pyproject.toml poetry.lock /back/
COPY ./src /back/src

WORKDIR /back
# How to install poetry in Docker
# https://stackoverflow.com/questions/53835198/integrating-python-poetry-with-docker
ARG YOUR_ENV="production"

ENV YOUR_ENV=${YOUR_ENV} \
    PYTHONFAULTHANDLER=1 \
    PYTHONUNBUFFERED=1 \
    PYTHONHASHSEED=random \
    PIP_NO_CACHE_DIR=off \
    PIP_DISABLE_PIP_VERSION_CHECK=on \
    PIP_DEFAULT_TIMEOUT=100 \
    POETRY_VERSION=1.4.2

RUN pip install "poetry==$POETRY_VERSION"

# Install python environment
RUN poetry config virtualenvs.create false \
    && poetry install $(test "$YOUR_ENV" = production && echo "--only main") --no-interaction --no-ansi

# Run the server
WORKDIR /back/src

EXPOSE 8000

# CMD ["/back/db/create-pgrounting-db", "/back/db/2022-12_pgrouting-bdtopo-run.sql.template", "/back/db/2022-12_pgrouting-bdtopo-run.sql", "&&", "uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"]
# CMD ["./start.fly"]

CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"]
