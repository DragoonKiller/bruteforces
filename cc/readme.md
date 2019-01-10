

用于对C++语言的代码作文本替换.

用法: `cc name [Options]`

例如:

`cc ./x.cpp -o ./submit.cpp -i ./templates/`

* `name`
    
    转换文件名(包括后缀)为 `name` 的cpp代码.  
    使用如下格式替换:
    ```
    #bruteforces include "all.hpp"
    ```

* `-o [name]`
    
    指定输出文件名. 默认 `submit.cpp`.

* `-i [dir]`
    
    指定搜索 `include` 文件的目录. 如果不指定, 则会按照**源文件**所在目录搜索.

* `-u`
    
    不输出关于展开文件的信息注释.

* `-w`
    
    不删除所有源文件内的注释.