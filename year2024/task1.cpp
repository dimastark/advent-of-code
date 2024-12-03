#include <algorithm>
#include <iostream>
#include <unordered_map>
#include <vector>

int main() {
    std::vector<int> list1;
    std::vector<int> list2;

    std::unordered_map<int, int> count2;

    for (int i = 0; i < 1000; ++i) {
        int n1, n2;
        std::cin >> n1 >> n2;

        list1.push_back(n1);
        list2.push_back(n2);

        ++count2[n2];
    }

    std::sort(list1.begin(), list1.end());
    std::sort(list2.begin(), list2.end());

    long int distance = 0;

    for (int i = 0; i < 1000; ++i) {
        distance += std::abs(list1[i] - list2[i]);
    }

    std::cout << "First star: " << distance << std::endl;

    long int similarity = 0;

    for (int i = 0; i < 1000; ++i) {
        similarity += list1[i] * count2[list1[i]];
    }

    std::cout << "Second star: " << similarity << std::endl;

    return 0;
}
