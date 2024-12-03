#include <iostream>
#include <regex>
#include <string>

int main() {
    using re_iterator = std::sregex_iterator;

    std::string expr;
    std::string line;

    while (getline(std::cin, line)) {
        expr += line;
    }

    long int sum = 0;
    long int cond_sum = 0;
    bool condition = true;

    std::regex re("(mul\\((\\d{1,3}),(\\d{1,3})\\)|do\\(\\)|don't\\(\\))");
    for (auto it = re_iterator(expr.begin(), expr.end(), re); it != re_iterator(); ++it) {
        auto match = *it;

        auto command = match[1].str();

        if (command.starts_with("don't")) {
            condition = false;
        } else if (command.starts_with("do")) {
            condition = true;
        } else if (command.starts_with("mul")) {
            auto a = std::stoi(match[2].str());
            auto b = std::stoi(match[3].str());

            sum += a * b;
            cond_sum += condition * (a * b);
        }
    }

    std::cout << "First star: " << sum << std::endl;
    std::cout << "Second star: " << cond_sum << std::endl;

    return 0;
}
