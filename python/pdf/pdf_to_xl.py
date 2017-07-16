#!/usr/bin/python
#-*- coding: utf-8 -*-

from pdfminer.converter import PDFPageAggregator
from pdfminer.pdfparser import PDFParser
from pdfminer.pdfdocument import PDFDocument
from pdfminer.pdfpage import PDFPage
from pdfminer.pdfpage import PDFTextExtractionNotAllowed
from pdfminer.pdfinterp import PDFResourceManager
from pdfminer.pdfinterp import PDFPageInterpreter
from pdfminer.layout import *
import re

#打开一个pdf文件
fp = open("test.pdf", 'rb')
#创建一个PDF文档解析器对象
parser = PDFParser(fp)
#创建一个PDF文档对象存储文档结构
#提供密码初始化，没有就不用传该参数
password = ""
document = PDFDocument(parser, password)
#document = PDFDocument(parser)
#检查文件是否允许文本提取
if not document.is_extractable:
    raise PDFTextExtractionNotAllowed
#创建一个PDF资源管理器对象来存储共享资源
#caching = False不缓存
rsrcmgr = PDFResourceManager(caching = False)
# 创建一个PDF设备对象
laparams = LAParams()
# 创建一个PDF页面聚合对象
device = PDFPageAggregator(rsrcmgr, laparams=laparams)
#创建一个PDF解析器对象
interpreter = PDFPageInterpreter(rsrcmgr, device)
#处理文档当中的每个页面

# doc.get_pages() 获取page列表
#for i, page in enumerate(document.get_pages()):
#PDFPage.create_pages(document) 获取page列表的另一种方式
replace=re.compile(r'\s+');
# 循环遍历列表，每次处理一个page的内容
flag = False
content = ""
end_flag = "2566-3328"
for page in PDFPage.create_pages(document):
    print(page)
    interpreter.process_page(26)
    # 接受该页面的LTPage对象
    layout=device.get_result()
    # 这里layout是一个LTPage对象 里面存放着 这个page解析出的各种对象
    # 一般包括LTTextBox, LTFigure, LTImage, LTTextBoxHorizontal 等等
    for x in layout:
        #如果x是水平文本对象的话
        if(isinstance(x,LTTextBoxHorizontal)):
            text=re.sub(replace,'|||',x.get_text())
            if len(text)!=0:
                if r"HK|||Stock|||(HKD)" in text:
                    flag = True
                elif end_flag in text:
                    flag = False
            if flag:
                print(text)

from xlwt import *
w = Workbook()
ws = w.add_sheet('hk_data')
ws.write(0,0,"text")
w.save('mini.xls')