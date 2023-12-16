# jinspect  

Simple project to analyze Java class file structure for educational purposes.

## Objectives
- Learning internals of java class file structure.
- File IO, Enums, binary operations in Rust
- Using command line arguments
- ...

## Quick Start

Compile java file:
```console
$ javac App.java
```

Run project:
```console
$ cargo run 
```

Example ouput for [App.java](./App.java) file:
```
INFO: ConstantPool= 29
 001 Method => ClassIndex: 2, NatIndex: 3
 002 Class => Index: 4
 003 NameAndType => NameIndex: 5, DescIndex: 6
 004 UTF8 => Value: java/lang/Object
 005 UTF8 => Value: <init>
 006 UTF8 => Value: ()V
 007 Field => ClassIndex: 8, NatIndex: 9
 008 Class => Index: 10
 009 NameAndType => NameIndex: 11, DescIndex: 12
 010 UTF8 => Value: java/lang/System
 011 UTF8 => Value: out
 012 UTF8 => Value: Ljava/io/PrintStream;
 013 String => Index: 14
 014 UTF8 => Value: hello, from java
 015 Method => ClassIndex: 16, NatIndex: 17
 016 Class => Index: 18
 017 NameAndType => NameIndex: 19, DescIndex: 20
 018 UTF8 => Value: java/io/PrintStream
 019 UTF8 => Value: println
 020 UTF8 => Value: (Ljava/lang/String;)V
 021 Class => Index: 22
 022 UTF8 => Value: App
 023 UTF8 => Value: Code
 024 UTF8 => Value: LineNumberTable
 025 UTF8 => Value: main
 026 UTF8 => Value: ([Ljava/lang/String;)V
 027 UTF8 => Value: SourceFile
 028 UTF8 => Value: App.java
INFO: Methods= 2
    Method: 00 - AF: 1, Name: <init> DI: 6
MaxStack: 1, MaxLocals: 1, CodeLen: 5, Array: [42, 183, 0, 1, 177]
LineNumberTable= start: 0, lineNumber: 1
    Method: 01 - AF: 9, Name: main DI: 26
MaxStack: 2, MaxLocals: 1, CodeLen: 9, Array: [178, 0, 7, 18, 13, 182, 0, 15, 177]
LineNumberTable= start: 0, lineNumber: 3
LineNumberTable= start: 8, lineNumber: 4
INFO: Attributes= 1
    Attribute: 00 - SourceFile: App.java

```

## Tasks
- [ ] Accept different class files via command line argument
- [ ] Choose operation (disassemble, signature, source) via command line 
- [ ] ...

## References
* Java Class File Structure: https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html
* Java Virtual Machine Instruction Set: https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-6.html
* JelloVM (Hello JVM in Python): https://youtube.com/playlist?list=PLpM-Dvs8t0VZ80zo4mwNKd9utc4vR7wUs&si=Knkr3Nape253SP6P
* Java Bytecode Crash Course: https://youtu.be/e2zmmkc5xI0?si=5UXE9T-7Gz6JINrL
