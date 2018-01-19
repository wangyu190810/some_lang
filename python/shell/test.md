

```python
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

print(foo("abcdaccd","a?cd*d"))
```

```sql

CREATE TABLE `douban_push` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `content_type` TINYINT(2) NOT NULL COMMENT '推荐类型',
  `link` VARCHAR(100) NOT NULL COMMENT '推荐链接',
  `link_id` INT(11) NOT NULL COMMENT '推荐文件文章id',
  `push_uid` INT(11) NOT NULL COMMENT '推荐者uid',
  `system_time` INT(11) NOT NULL COMMENT '推荐时间',
  PRIMARY KEY (`id`));

ALTER TABLE `douban_push` 
ADD INDEX `uid` (`push_uid` ASC),
ADD INDEX `link_id` (`link_id` ASC);

SELECT uid FROM test.douban_push where link_id = 123123;
SELECT * FROM test.douban_push where uid = 1111 order by id desc;

```