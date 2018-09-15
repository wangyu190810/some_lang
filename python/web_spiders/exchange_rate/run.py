from lib.fileprocess import data_export_process

def main():
    data = data_export_process("./run/urls.xlsx")
    print(data)
    
if __name__ == '__main__':
    main()