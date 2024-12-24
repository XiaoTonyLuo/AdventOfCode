#include <iostream>
#include <vector>
#include <algorithm>
#include <cstdio>
#include <sstream>

std::vector<std::vector<int>> processInput()
{
    std::vector<std::vector<int>> input;
    std::string line_input;
    while(std::getline(std::cin, line_input))
    {
        if(!line_input.empty())
        {
            std::vector<int> report;
            std::stringstream line_stream (line_input);
            int number;
            while(line_stream >> number)
            {
                report.push_back(number);
            }
            input.push_back(report);
        }
    }
    return input;
}

bool is_report_safe(const std::vector<int>& report)
{
    bool is_increasing = std::adjacent_find(report.begin(), report.end(), std::greater_equal<int>()) == report.end();
    bool is_decreasing = std::adjacent_find(report.rbegin(), report.rend(), std::greater_equal<int>()) == report.rend();

    if(!is_increasing && !is_decreasing) return false;

    for(int i = 0; i + 1 < report.size(); ++i)
    {
        if(std::abs(report[i + 1] - report[i] ) > 3)
        {
            return false;
        }
    }
    return true;
}

long long part1(const std::vector<std::vector<int>>& input)
{
    long long safe_reports = 0;
    for(const auto &report : input)
    {
        if(is_report_safe(report))
        {
            ++safe_reports;
        }
    }
    return safe_reports;
}
long long part2(const std::vector<std::vector<int>>& input)
{
    long long safe_reports = 0;
    for(const auto &report : input)
    {
        for(int i = 0; i < report.size(); ++i)
        {
            std::vector<int> report_remove_element = report;
            report_remove_element.erase(report_remove_element.begin() + i);
            if(is_report_safe(report_remove_element))
            {
                ++safe_reports;
                break;
            }
        }
    }
    return safe_reports;
}

int main(int argc, char** argv)
{
    if(argc == 1)
        std::freopen("day2inp.txt", "r", stdin);
    else
        std::freopen(argv[1], "r", stdin);

    std::vector<std::vector<int>>  input = processInput();

    std::cout << "The solution to part 1 is: " << part1(input) << '\n';
    std::cout << "The solution to part 2 is: " << part2(input) << '\n';
}