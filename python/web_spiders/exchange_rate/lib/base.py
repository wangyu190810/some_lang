import tornado

import tornado.ioloop
import tornado.web
from tornado.httpclient  import  AsyncHTTPClient

import time
from datetime import timedelta

url = "www.22too.com"

try:
    from HTMLParser import HTMLParser
    from urlparse import urljoin, urldefrag
except ImportError:
    from html.parser import HTMLParser
    from urllib.parse import urljoin, urldefrag


class BaseHandler(tornado.web.RequestHandler):
    pass

class MainHandler(BaseHandler):
    def get(self):
        self.write("Hello, world")

def handle_response(response):
    print(response)
    if response.error:
        print("Error: %s" % response.error)
    else:
        print(response.body)

def get_data(url):

    print('fetched %s' % url)
    response = yield AsyncHTTPClient().fetch(url)
    print('fetched %s' % url)
    print(response)
    html = response.body if isinstance(response.body, str) \
        else response.body.decode(errors='ignore')
    get_links(html)

def get_links(html):
    class URLSeeker(HTMLParser):
        def __init__(self):
            HTMLParser.__init__(self)
            self.urls = []

        def handle_starttag(self, tag, attrs):
            href = dict(attrs).get('href')
            if href and tag == 'a':
                self.urls.append(href)

    url_seeker = URLSeeker()
    url_seeker.feed(html)
    return url_seeker.urls

if __name__ == '__main__':
    print("start")
    for row in get_data(url):
        print(row)