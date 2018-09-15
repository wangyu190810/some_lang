def foo(match_str,flag_str):
    match_flag = False
    _str = "a?cd*d"
    question_mark = flag_str.find("?")
    if question_mark > 0:
        question_mark_flag = True
        print(question_mark_flag) 
        print(question_mark) 
    exclamation_mark = flag_str.find("*")
    if exclamation_mark >0:
        exclamation_mark_flag = True 
        print(exclamation_mark_flag ) 
        print(exclamation_mark ) 
    if exclamation_mark_flag and question_mark_flag and (exclamation_mark > question_mark):
        print(flag_str[:question_mark])
        print(flag_str[question_mark:exclamation_mark])
        if flag_str[:question_mark-1] in match_str and flag_str[question_mark:exclamation_mark_flag-1] in match_str:
            match_flag = True
    return match_flag

def foo(match_str,flag_str):
    """
    match_str:匹配字符
    falg_str:通配符串
    """

    match_flag = False
    _str = "a?cd*d"
    question_mark = flag_str.find("?")
    if question_mark > 0:
        question_mark_flag = True
    exclamation_mark = flag_str.find("*")
    if exclamation_mark >0:
        exclamation_mark_flag = True 
    if exclamation_mark_flag and question_mark_flag and (exclamation_mark > question_mark):
        if flag_str[:question_mark-1] in match_str and flag_str[question_mark:exclamation_mark_flag-1] in match_str:
            match_flag = True
    return match_flag


print(foo("abcdaccd","a?cd*dd"))

