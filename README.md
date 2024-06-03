# -Kumir-Helper
VERSION: 1.0;


**Syntax, Синтаксис:** 
  1) { english: "using (draftsman)" = russian: "использовать Чертежник"},
  2) { english: "pen_down" = russian: "опустить перо"},
  3) { english: "pen_up" = russian: "поднять перо"},
  4) { english: "main" = russian: "алг" },
  5) { english: "start" = russian: "нач" },
  6) { english: "end" = russian: "кон" },
  7) { english: "moveTo()" = russian: "сместиться в точку()" },
  8) { english: "vectorTo()" = russian: "сместиться на вектор()" },

**Ru:**
-Kumir-Helper -> это консольная программа (транслятор) написанная на языке Rust, которая создана что-бы помогать людям писать код для 'Кумира'.
  1) Вы пишете код на английском языке в блокноте.
  2) Используя консоль вы пишете абсолютный путь до файла (input и output (все файлы должны быть .kum))
  3) Если это необходимо, задаёте параметры для moveTo() и подобных
  4) Код "транслируется" с английского языка на язык для кумира (из input в output)
  5) Открыавете output.kum в кумире
     
**Eng:**
-Kumir-Helper -> is a console program (translator) written in the Rust language, which was created to help people write code for 'Idol'.
  1) You write code in English in Notepad.
  2) Using the console you write the absolute path to the file (input and output (all files must be .kum))
  3) If necessary, set parameters for moveTo() and similar
  4) The code is “translated” from English into the language for the idol (from input to output)
  5) Open output.kum in idol

**Ru Tutorial:**

Создайте два файла input (куда будете вводить код на английском) и output (куда будет транслироваться код) оба файла должны быть .kum формата.

Я использую windows 11 => то что эта программа работает на MacOS или Linux не обещаю.
Открываем файл (ПКМ > Открыть с помошью > Блокнот)
![OpenFile](https://github.com/Gvinses/-Kumir-Helper/assets/158508670/c14cdcff-2489-4617-9854-3b382d8af488)

Пишем код в соотвествии с синтаксисом (не вписывая значения в скобочки где они могут быть т.е moveTo() оставляем без значений в скобочках).
![image](https://github.com/Gvinses/-Kumir-Helper/assets/158508670/bf9442cd-d457-4956-b2f4-c998e72cebc7)

Как получить путь файла? Нажимаем ЛКМ по D:\\ (у вас может быть путь другой) нужен имменно тот путь, где находятся файлы input и output.
![image](https://github.com/Gvinses/-Kumir-Helper/assets/158508670/7b61a593-e2dc-46c3-82c3-1356f0570bdb)

Вводим абсолютный путь до файла (от диска до самого файла с расширением файла (ОБЯЗАТЕЛЬНО!)) 
![image](https://github.com/Gvinses/-Kumir-Helper/assets/158508670/c495a03b-57bb-4efd-ba69-9d47199c4471)

Если в вашем коде присутствует moveTo(), то программа спросит их значения. Вводите их через запятую без пробела: x,y => например => 3,2
![image](https://github.com/Gvinses/-Kumir-Helper/assets/158508670/170286f2-bc44-48f7-95f2-d7c630ebeb64)

Если вы всё сделали правильно, программа закроется, а в файле, который вы указали как output при его открытии появится код для кумира.
![image](https://github.com/Gvinses/-Kumir-Helper/assets/158508670/463adcfc-b3b4-43af-adb4-259102e1a38d)

Заходим в кумир. Нажимаем открыть файл. И выбираем наш output файл.
![image](https://github.com/Gvinses/-Kumir-Helper/assets/158508670/97fc2bee-e523-4477-b5da-b1f7a02237dd)

Всё вставится и вы можете наслаждаться вашей работай.
![image](https://github.com/Gvinses/-Kumir-Helper/assets/158508670/f77c929d-f28e-4755-9f5b-869e53aac7e2)



**Eng Tutorial:**

Create two files input (where you will enter the code in English) and output (where the code will be translated), both files should be in .kum format.

I use windows 11 => I can’t promise that this program works on MacOS or Linux.
Open the file (RMB > Open with > Notepad)
![OpenFile](https://github.com/Gvinses/-Kumir-Helper/assets/158508670/c14cdcff-2489-4617-9854-3b382d8af488)

We write the code in accordance with the syntax (without entering values ​​in brackets where they may be, i.e. moveTo() is left without values ​​in brackets).
![image](https://github.com/Gvinses/-Kumir-Helper/assets/158508670/bf9442cd-d457-4956-b2f4-c998e72cebc7)

How to get the file path? Click LMB on D:\\ (you may have a different path) you need exactly the path where the input and output files are located.
![image](https://github.com/Gvinses/-Kumir-Helper/assets/158508670/7b61a593-e2dc-46c3-82c3-1356f0570bdb)

Enter the absolute path to the file (from the disk to the file itself with the file extension (MANDATORY!))
![image](https://github.com/Gvinses/-Kumir-Helper/assets/158508670/c495a03b-57bb-4efd-ba69-9d47199c4471)

If moveTo() is present in your code, the program will ask for their values. Enter them separated by commas without spaces: x,y => for example => 3,2
![image](https://github.com/Gvinses/-Kumir-Helper/assets/158508670/170286f2-bc44-48f7-95f2-d7c630ebeb64)

If you did everything correctly, the program will close, and the code for the idol will appear in the file that you specified as output when you open it.
![image](https://github.com/Gvinses/-Kumir-Helper/assets/158508670/463adcfc-b3b4-43af-adb4-259102e1a38d)

Let's go to the idol. Click open file. And select our output file.
![image](https://github.com/Gvinses/-Kumir-Helper/assets/158508670/97fc2bee-e523-4477-b5da-b1f7a02237dd)

Everything will fit in and you can enjoy your work.
![image](https://github.com/Gvinses/-Kumir-Helper/assets/158508670/f77c929d-f28e-4755-9f5b-869e53aac7e2)
