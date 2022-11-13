# Documentation link

http://www.hug.rest/website/quickstart

# Dependencies

```sh
sudo apt install uwsgi-core uwsgi-plugin-python3
```

# Launch the example

```sh
uwsgi --http-socket 0.0.0.0:8000 --plugin /usr/lib/uwsgi/plugins/python310_plugin.so --wsgi-file first-api.py --callable __hug_wsgi__
uwsgi --http-socket 0.0.0.0:8000 --plugin python310 --wsgi-file first-api.py --callable __hug_wsgi__
uwsgi --http-socket 0.0.0.0:8000 --plugin python310 --wsgi-file first-api.py
```

Check if the python plugin is correctly found ([documentation](https://stackoverflow.com/questions/65362467/uwsgi-cant-find-python3-plugin-open-python3-plugin-so-no-such-file-or-d)):

```sh
uwsgi --plugin python310 -s :0
```

Use a FIFO socket to communicate with the `uwsgi` program:

```sh
uwsgi --http-socket 0.0.0.0:8000 --plugin python310 --wsgi-file first-api.py --master-fifo /tmp/uwsgi
```

While the FIFO socket is available, to restart it, I can launch in another terminal ([documentation_0](https://uwsgi-docs.readthedocs.io/en/latest/articles/TheArtOfGracefulReloading.html) [documentation_1](https://uwsgi-docs.readthedocs.io/en/latest/MasterFIFO.html)):

```sh
echo r > /tmp/uwsgi
```
