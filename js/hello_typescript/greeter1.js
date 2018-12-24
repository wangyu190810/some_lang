function greeter(person) {
    return "hello , " + person.firstName + person.lastName;
}
var user = { firstName: "22", lastName: "too" };
document.body.innerHTML = greeter(user);
