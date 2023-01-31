# Bridge Pattern 

https://stackoverflow.com/a/10689017

When:
```
                   ----Shape---
                  /            \
         Rectangle              Circle
        /         \            /      \
BlueRectangle  RedRectangle BlueCircle RedCircle
```
Refactor to:
```
          ----Shape---                        Color
         /            \                       /   \
Rectangle(Color)   Circle(Color)           Blue   Red
```
桥接模式通过将继承改为组合的方式来解决这个问题。 具体来说， 就是抽取其中一个维度并使之成为独立的类层次， 这样就可以在初始类中引用这个新层次的对象， 从而使得一个类不必拥有所有的状态和行为。

More examples:

RadioRemote / TVRemote / .....


# What does bridge pattern do

我们把类拆分成分锅的类和干活的类。非常像小学时的坏班长从来不干活的哦

分锅的类基本上是GUI，也就是图形用户界面，都不管哪个类来干，调接口就完事了。实现部分呢？底层操作系统的API代码。

所以哦，就是你可以开发不同的GUI，同时支持不同的API。
