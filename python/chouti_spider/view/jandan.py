from utils.net_wrok import headers
import requests



def request(url):
    data = requests.get(url = url, headers=headers)
    return data

def hot():
    pass
