
# Singleton
* Singleton is a creational design pattern that lets you ensure that a class has only one instance, while providing a global access point to this instance.

- Ensure Only 1 instance.
- Global Access

## pros
Hardly I can find out any reason why shouldnt we use Singleton.

Its common that a class only needs one object to handle all the requests. For example, an service in MVC to control database.

## cons
you don’t want the code to be scattered all over your program.

It’s much better to have it within one class, especially if the rest of your code already depends on it.

## How to build

In golang just sync.Once()

Java:

