

用于对C++语言的代码作文本替换.

用法: `cc name [Options]`

例如:

`cc ./x.cpp -o ./submit.cpp -i ./templates/`

* `name`
    
    转换文件名(包括后缀)为 `name` 的cpp代码.  
    代码中必须有如下注释中的两行识别行(行内字符必须完全一致, 包括空格), 其中使用双引号的 `include` 会被直接替换成文件内容, 默认其它行会被忽略. 支持递归替换.
    ```
    /* bruteforces generation begin */
    #include "all.hpp"
    /* briteforces generation end */
    ```

* `-o [name]`
    
    指定输出文件名. 默认 `submit.cpp`.

* `-i [dir]`
    
    指定搜索 `include` 文件的目录. 如果不指定, 则会按照**源文件**所在目录搜索.

* `-u`
    
    不输出关于展开文件的信息注释.

* '-g'
    
    不忽略处于展开块内的, 不能匹配 `#include "..."` 的语句. 