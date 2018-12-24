import {catService,catProd} from "./Config"

class Student{
    fullName : String;

    constructor(public firstName:String,public middleInitial:String, public lastName:String){
        this.fullName = firstName + " " + middleInitial + " " + lastName;
    }

}
interface Person{
    firstName:String,
    lastName:String,

}

function greeter(person:Person){
    catProd.info(()=>"lastname"+ person.lastName);
    catProd.info(()=>"firstname"+ person.firstName);

    return "Hello " + person.firstName + person.lastName
}


let user = new Student("Jane.","M","User");

document.body.innerHTML = greeter( user);