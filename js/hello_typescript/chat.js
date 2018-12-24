const content = request.params["content"];
const content_object = JSON.parse(content);

var pro_async = new Promise(function(resolve, reject)  {
    if (content_object["_lctype"] == 127) {
    var requestk = require('request');
    var url = "https://sztm42qu.api.lncld.net/1.1/rtm/messages";
    var option = {
        url: url,
        method: "POST",
        json: true,

        headers: {
            "X-LC-Id": "SzTm42qulIXv7p9igWpCnbDG-gzGzoHsz",
            "X-LC-Key": "wM32Mk9SpsmyvIQn6S7zh7Yn,master",
            "Content-Type": "application/json"
        },
        body: {
            'from_peer': 'ruiboma',
            'message': {
                "_lctype": 101,
                "_lctext": "消息",
                "_lcattrs": {"html": "我是智能小客服犇犇，很高兴为您服务，大部分问题我都会哦；您可以点击下方链接了解相关问题。<br><a href='%7b%22common%22%3a%7b%22type%22%3a%220%22%2c%22data%22%3a%7b%22key%22%3a%22value%22%7d%7d%2c%22mobile%22%3a%7b%22type%22%3a%220%22%2c%22text%22%3a%22111%22%2c%22data%22%3a%7b%22s%22%3a%22m%22%7d%7d%7d'>112相关?</a><br><a href=\"\">资金相关?</a><br><a href=\"\">开户相关?</a><br><a href='%7b%22common%22%3a%7b%22type%22%3a%220%22%2c%22data%22%3a%7b%22key%22%3a%22value%22%7d%7d%2c%22mobile%22%3a%7b%22type%22%3a%221%22%2c%22text%22%3a%22111%22%2c%22data%22%3a%7b%22url%22%3a%22schemeholder%3a%2f%2fmsgBox%22%7d%7d%7d'>资金相关?</a><br><a href='%7b%22common%22%3a%7b%22type%22%3a%220%22%2c%22data%22%3a%7b%22key%22%3a%22value%22%7d%7d%2c%22mobile%22%3a%7b%22type%22%3a%221%22%2c%22text%22%3a%22111%22%2c%22data%22%3a%7b%22s%22%3a%22m%22%7d%7d%7d'>开户相关?</a><br>如遇到暂时无法解答的问题，您可以转人工客服进行咨询。"}
            }
            ,
            'conv_id': request.params['convId']
        }
    };
    requestk(option, function (error, response, body) {
        if (!error && response.statusCode == 200) {
            console.log(body);
            // reject(AV.Cloud.Error)
            resolve()
        }else {
            reject()
        }
    });

}});
return pro_async;
