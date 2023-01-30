# Adapters
Why I study it?

It is struggling to combine different processor functions with different input&output types to the same object ( Object was handled with different functions, one by one), but when we also hope each processor function can be activated or inactivated flexibly.

## Example
https://refactoringguru.cn/design-patterns/adapter/java/example

How to understand it? 
```
Hole   --> Peg
            △
            |
Square <- PegAdapter
```

*// 2023-01-30 additions*

### What the chart says?

YOU are responsible for a client service, It is time for you to call some 3rd-party service or sdk etc. But the different interface (the type of data,I mean) stops you from writing code comfortably. 

Cons:

Class++, Interface++

Sometimes it's **simpler** to modify the service to **match** client.

*// 2023-01-30 additions*
 
```
Client --> <<interface>>
           Client Interface
           + method(data)
                △
                |
                |
             Adapter
         - Adaptee:Service --------> Service
         + method(data)
```
Client <-> Hole

Client Interface <-> getRadius()

Adapter <-> Extend from `Client`, With an object of `Service`, also with the convert function like getRadius()

Service <-> `Square`, maybe an 3rd party package we meet in reality,






expected effect
``` java
Hole.CheckIfFits(PegAdapter(Square))
```

# 应用场景
1. 第三方代码无法修改（第三方，许可条款...... 其他内容）
2. 如果能修改类就不需要使用适配器了，会增加代码的整体复杂度

# 优点 
1. 单一职责原则 类可以专心自己的事情
2. 开闭原则 不修改 Hole 与 data， 在这里就是不修改 Client 代码的情况下添加其他的适配器
