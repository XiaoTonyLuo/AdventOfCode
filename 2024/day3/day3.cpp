#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>
#include <regex>

int part_one(const std::vector<std::string> &data)
{
    const std::regex pattern(R"(mul\((\d{1,3}),(\d{1,3})\))");

    int result = 0;
    for (auto line : data)
    {
        std::string data_copy = line;
        for (std::smatch sm; std::regex_search(data_copy, sm, pattern); data_copy = sm.suffix())
        {
            result += std::stoi(sm[1]) * std::stoi(sm[2]);
        }
    }
    return result;
}

int part_two(const std::vector<std::string> &data)
{
    auto result = 0;
    const std::regex pattern(R"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))");
    bool ignore = false;
    for (auto line : data)
    {
        std::string data_copy = line;
        for (std::smatch sm; std::regex_search(data_copy, sm, pattern); data_copy = sm.suffix())
        {
            // std::cout << sm[0] << std::endl;
            if (sm[0].str() == "do()")
            {
                ignore = false;
                continue;
            }
            else if (sm[0].str() == "don't()")
            {
                ignore = true;
                continue;
            }

            if (!ignore)
            {
                auto tmp = std::stoi(sm[1]) * std::stoi(sm[2]);
                std::cout << "Result for: " << sm[0] << "is " << tmp << std::endl;
                result += tmp;
            }
        }
    }
    return result;
}

int main()
{
    std::ifstream file("./input.txt");
    std::string line;
    std::vector<std::string> data;
    while (std::getline(file, line))
    {
        data.push_back(line);
    }

    std::cout << "Part one: " << part_one(data) << std::endl;
    std::cout << "Part two: " << part_two(data) << std::endl;
    return 0;
}