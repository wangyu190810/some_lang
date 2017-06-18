using System;

namespace Design_pattern{


public class Operation{

    private double _numberA = 0;
    private double _numberB = 0;

    public double NumberA{
        get
        {
            return _numberA;
        }
        set{
            _numberA = value;
        }

    }
    public double NumberB{
        get
        {
            return _numberB;
        }
        set{
            _numberB = value;
        }

    }

    public virtual double GetResutl(){
        double result  = 0;
        return result;
    }



}


class OperationAdd: Operation{

    public override double GetResutl(){
        double result = 0;
        result = NumberA + NumberB;
        return result;
    }
}


class OperationSub: Operation{

    public override double GetResutl(){
        double result = 0;
        result = NumberA - NumberB;
        return result;
    }
}


class OperationMul: Operation{

    public override double GetResutl(){
        double result = 0;
        result = NumberA * NumberB;
        return result;
    }
}


class OperationDiv: Operation{

    public override double GetResutl(){
        double result = 0;
        if(NumberB == 0){
            throw new Exception("除数不等于零");
        }
        result = NumberA / NumberB;
        return result;
    }
}

class OperationFactory{
   public static Operation createOperation(string  Operate){
       Operation oper = null;
       switch(Operate){
           case "+":
                oper = new OperationAdd();
                break;
            case "-":
                oper = new OperationSub();
                break;
            case "*": 
                oper = new OperationMul();
                break;
            case "/":
                oper = new OperationDiv();
                break;
            default:
                throw new Exception("输入错误重新输入");
       }
       return oper;
   }


}

class OperateTest{
    public void run(){
        Operation oper;
        oper = OperationFactory.createOperation("-");
        oper.NumberA = 6;
        oper.NumberB = 101;
        oper.GetResutl();
        Console.WriteLine(oper.GetResutl());

    } 
}

}

