

用于对C++语言的代码作文本替换.

用法: `cc [Options]`


参数:

`cc name [Options]`

例如:

`cc ./x.cpp -o ./submit.cpp -i ./templates/`

* `name`
    
    转换文件名(包括后缀)为 `name` 代码.  
    代码中必须有如下注释中的两行识别行(行内字符必须完全一致, 包括空格), 其中使用双引号的 `include` 会被替换成文件内容. 不支持递归替换.
    ```
    /* bruteforces generation begin */
    #include "all.hpp"
    /* briteforces generation end */
    ```

* `-o [name]`
    
    指定输出文件名. 默认 `submit.cpp`.

* `-i [dir]`
    
    指定搜索 `include` 文件的目录. 如果不指定, 则会按照源文件所在目录搜索.