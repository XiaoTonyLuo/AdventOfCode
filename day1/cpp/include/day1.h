#include <fstream>
#include <vector>
#include <iterator>

using std::ifstream;
using std::vector;

size_t Calculate(ifstream& fin)
{
    vector<int> data{};
    std::copy(std::istream_iterator<int>(fin), std::istream_iterator<int>(), std::back_inserter(data));
    size_t counter = 0;
    for (size_t i = 1; i < data.size(); ++i)
    {
        if (data[i] > data[i - 1])
        {
            ++counter;
        }
    }
    return counter;
}