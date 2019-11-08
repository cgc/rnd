Classfile jar:file:/Users/carlos/me/prog3/Critter.jar!/assignment/Critter.class
  Last modified Sep 15, 2017; size 901 bytes
  MD5 checksum 92512a5e71444239fd4a8ff57b69c3b0
  Compiled from "Critter.java"
public interface assignment.Critter
  minor version: 0
  major version: 52
  flags: ACC_PUBLIC, ACC_INTERFACE, ACC_ABSTRACT
Constant pool:
   #1 = Class              #2             // assignment/Critter
   #2 = Utf8               assignment/Critter
   #3 = Class              #4             // java/lang/Object
   #4 = Utf8               java/lang/Object
   #5 = Utf8               FRONT
   #6 = Utf8               I
   #7 = Utf8               ConstantValue
   #8 = Integer            0
   #9 = Utf8               FRONT_RIGHT
  #10 = Integer            45
  #11 = Utf8               RIGHT
  #12 = Integer            90
  #13 = Utf8               REAR_RIGHT
  #14 = Integer            135
  #15 = Utf8               REAR
  #16 = Integer            180
  #17 = Utf8               REAR_LEFT
  #18 = Integer            225
  #19 = Utf8               LEFT
  #20 = Integer            270
  #21 = Utf8               FRONT_LEFT
  #22 = Integer            315
  #23 = Utf8               EMPTY
  #24 = Utf8               WALL
  #25 = Integer            1
  #26 = Utf8               ENEMY
  #27 = Integer            2
  #28 = Utf8               ALLY
  #29 = Integer            3
  #30 = Utf8               BAD
  #31 = Integer            -1
  #32 = Utf8               REGISTERS
  #33 = Integer            10
  #34 = Utf8               WELL_FED_DURATION
  #35 = Integer            30
  #36 = Utf8               getCode
  #37 = Utf8               ()Ljava/util/List;
  #38 = Utf8               getNextCodeLine
  #39 = Utf8               ()I
  #40 = Utf8               setNextCodeLine
  #41 = Utf8               (I)V
  #42 = Utf8               getReg
  #43 = Utf8               (I)I
  #44 = Utf8               setReg
  #45 = Utf8               (II)V
  #46 = Utf8               hop
  #47 = Utf8               ()V
  #48 = Utf8               left
  #49 = Utf8               right
  #50 = Utf8               eat
  #51 = Utf8               infect
  #52 = Utf8               getCellContent
  #53 = Utf8               getOffAngle
  #54 = Utf8               ifRandom
  #55 = Utf8               ()Z
  #56 = Utf8               SourceFile
  #57 = Utf8               Critter.java
{
  public static final int FRONT;
    descriptor: I
    flags: ACC_PUBLIC, ACC_STATIC, ACC_FINAL
    ConstantValue: int 0

  public static final int FRONT_RIGHT;
    descriptor: I
    flags: ACC_PUBLIC, ACC_STATIC, ACC_FINAL
    ConstantValue: int 45

  public static final int RIGHT;
    descriptor: I
    flags: ACC_PUBLIC, ACC_STATIC, ACC_FINAL
    ConstantValue: int 90

  public static final int REAR_RIGHT;
    descriptor: I
    flags: ACC_PUBLIC, ACC_STATIC, ACC_FINAL
    ConstantValue: int 135

  public static final int REAR;
    descriptor: I
    flags: ACC_PUBLIC, ACC_STATIC, ACC_FINAL
    ConstantValue: int 180

  public static final int REAR_LEFT;
    descriptor: I
    flags: ACC_PUBLIC, ACC_STATIC, ACC_FINAL
    ConstantValue: int 225

  public static final int LEFT;
    descriptor: I
    flags: ACC_PUBLIC, ACC_STATIC, ACC_FINAL
    ConstantValue: int 270

  public static final int FRONT_LEFT;
    descriptor: I
    flags: ACC_PUBLIC, ACC_STATIC, ACC_FINAL
    ConstantValue: int 315

  public static final int EMPTY;
    descriptor: I
    flags: ACC_PUBLIC, ACC_STATIC, ACC_FINAL
    ConstantValue: int 0

  public static final int WALL;
    descriptor: I
    flags: ACC_PUBLIC, ACC_STATIC, ACC_FINAL
    ConstantValue: int 1

  public static final int ENEMY;
    descriptor: I
    flags: ACC_PUBLIC, ACC_STATIC, ACC_FINAL
    ConstantValue: int 2

  public static final int ALLY;
    descriptor: I
    flags: ACC_PUBLIC, ACC_STATIC, ACC_FINAL
    ConstantValue: int 3

  public static final int BAD;
    descriptor: I
    flags: ACC_PUBLIC, ACC_STATIC, ACC_FINAL
    ConstantValue: int -1

  public static final int REGISTERS;
    descriptor: I
    flags: ACC_PUBLIC, ACC_STATIC, ACC_FINAL
    ConstantValue: int 10

  public static final int WELL_FED_DURATION;
    descriptor: I
    flags: ACC_PUBLIC, ACC_STATIC, ACC_FINAL
    ConstantValue: int 30

  public abstract java.util.List getCode();
    descriptor: ()Ljava/util/List;
    flags: ACC_PUBLIC, ACC_ABSTRACT

  public abstract int getNextCodeLine();
    descriptor: ()I
    flags: ACC_PUBLIC, ACC_ABSTRACT

  public abstract void setNextCodeLine(int);
    descriptor: (I)V
    flags: ACC_PUBLIC, ACC_ABSTRACT

  public abstract int getReg(int);
    descriptor: (I)I
    flags: ACC_PUBLIC, ACC_ABSTRACT

  public abstract void setReg(int, int);
    descriptor: (II)V
    flags: ACC_PUBLIC, ACC_ABSTRACT

  public abstract void hop();
    descriptor: ()V
    flags: ACC_PUBLIC, ACC_ABSTRACT

  public abstract void left();
    descriptor: ()V
    flags: ACC_PUBLIC, ACC_ABSTRACT

  public abstract void right();
    descriptor: ()V
    flags: ACC_PUBLIC, ACC_ABSTRACT

  public abstract void eat();
    descriptor: ()V
    flags: ACC_PUBLIC, ACC_ABSTRACT

  public abstract void infect();
    descriptor: ()V
    flags: ACC_PUBLIC, ACC_ABSTRACT

  public abstract void infect(int);
    descriptor: (I)V
    flags: ACC_PUBLIC, ACC_ABSTRACT

  public abstract int getCellContent(int);
    descriptor: (I)I
    flags: ACC_PUBLIC, ACC_ABSTRACT

  public abstract int getOffAngle(int);
    descriptor: (I)I
    flags: ACC_PUBLIC, ACC_ABSTRACT

  public abstract boolean ifRandom();
    descriptor: ()Z
    flags: ACC_PUBLIC, ACC_ABSTRACT
}
SourceFile: "Critter.java"
