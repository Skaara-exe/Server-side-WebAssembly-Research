# Instructions

## C++

Generate a new project folder and file

```
mkdir hello-world-cpp
cd hello-world-cpp
```

Inside VS Code, create a new file called `helloworld.cpp` and add:

```cpp
#include <iostream>
#include <vector>
#include <string>

using namespace std;

int main()
{
    vector<string> msg {"Hello", "World", "!"};

    for (const string& word : msg)
    {
        cout << word << " ";
    }
    cout << endl;
}
```

Compile and run the C++ code

```
g++ hello.cpp -o hello
./hello
```

**Note:** the ```-o ``` parameter is used to name the output file


source:
https://code.visualstudio.com/docs/cpp/config-mingw