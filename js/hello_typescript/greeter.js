"use strict";
exports.__esModule = true;
var Config_1 = require("./Config");
var Student = /** @class */ (function () {
    function Student(firstName, middleInitial, lastName) {
        this.firstName = firstName;
        this.middleInitial = middleInitial;
        this.lastName = lastName;
        this.fullName = firstName + " " + middleInitial + " " + lastName;
    }
    return Student;
}());
function greeter(person) {
    Config_1.catProd.info(function () { return "lastname" + person.lastName; });
    Config_1.catProd.info(function () { return "firstname" + person.firstName; });
    return "Hello " + person.firstName + person.lastName;
}
var user = new Student("Jane.", "M", "User");
document.body.innerHTML = greeter(user);
