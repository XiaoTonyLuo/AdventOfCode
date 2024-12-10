#include <iostream>
#include <fstream>
#include <string>
#include "day1.h"

using std::ifstream;
using std::string;

int main(int argc, char* argv[])
{
    if (argc != 2) {
        std::cerr << "Arguments number error! Need 1 argument, given " << argc - 1 << " argument(s)!";
        return -1;
    }
    string arg = argv[1];
    arg = "..\\..\\..\\..\\" + arg;

    ifstream fin;
    fin.open(arg);
    if (!fin)
    {
        std::cerr << "cannot open the file";
        return -1;
    }

    auto counter = Calculate(fin);
    std::cout << "The measurement incresed " << counter << " times!";
}
