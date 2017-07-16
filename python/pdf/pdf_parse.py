# -*- coding: utf-8 -*-
import PyPDF2
from PyPDF2 import PdfFileReader
import io  
import sys 
#a_type = sys.getfilesystemencoding()
reload(sys)
sys.setdefaultencoding("utf-8")
# print content.decode('utf-8').encode(type)
# sys.stdout = io.TextIOWrapper(sys.stdout.buffer,encoding='utf8')
lang = ['utf8',
'gbk',	
'gb2312',
'gb18030',
'big5',
'big5hkscs']


pdfFileObj = open('test.pdf', 'rb')
pdfReader = PyPDF2.PdfFileReader(pdfFileObj)
print(pdfReader.numPages)
pageObj = pdfReader.getPage(2)
# local_file = open("test.txt","w+")
text = pageObj.extractText()
# print(pageObj.extractText().encode("utf-8"))
print(type(text))

for row in lang:
    try:
        print(text.encode(row))
        # print(text)
        # text.encode(a_type)
        # print(text)
        # print(str(text))
    except Exception as e:
        print(e) 
# local_file.write(text)
# local_file.close()

