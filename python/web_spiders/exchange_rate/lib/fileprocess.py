import xlrd
import xlwt


def data_export_process(file_name, func=None):
    """
    读取不同xl数据
    :param file_name: 读取文件的名称
    :param func: 读取的函数，
    :return:
    """
    resp_data = []
    workbook = xlrd.open_workbook(file_name)
    sheet = workbook.sheet_by_index(0)
    if sheet.nrows == 0 and sheet.nrows == 0:
        pass
    sheet_data = []
    nrows = sheet.nrows
    ncols = sheet.ncols
    line_data = {}
    for row in range(nrows):
        if row in (0, 1):
            continue
        province = sheet.cell(row, 0).value
        name = sheet.cell(row, 1).value
        base_url = sheet.cell(row, 2).value
        sheet_data.append(
            dict(province=province, name=name, base_url=base_url))

    return sheet_data