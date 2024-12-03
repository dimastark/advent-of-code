#include <iostream>
#include <sstream>
#include <vector>

bool is_safe_report(const std::vector<int>& report) {
    bool is_asc = report[0] < report[1];

    for (size_t i = 1; i < report.size(); ++i) {
        auto diff = report[i] - report[i - 1];

        if (diff == 0 || std::abs(diff) > 3) {
            return false;
        }

        if (is_asc && diff < 0) {
            return false;
        }

        if (!is_asc && diff > 0) {
            return false;
        }
    }

    return true;
}

bool is_fixing_report(const std::vector<int>& report) {
    bool result = is_safe_report(report);

    for (size_t i = 0; i < report.size(); ++i) {
        auto tmp_report = report;
        tmp_report.erase(tmp_report.begin() + i);
        result = result || is_safe_report(tmp_report);
    }

    return result;
}

int main() {
    std::vector<std::vector<int>> reports;

    for (int i = 0; i < 1000; ++i) {
        std::string tok;
        getline(std::cin, tok);

        std::vector<int> report;

        std::stringstream ss(tok);
        while (getline(ss, tok, ' ')) {
            report.push_back(std::stoi(tok));
        }

        reports.push_back(std::move(report));
    }

    long int safe_reports_count = 0;
    long int fixing_reports_count = 0;

    for (const auto& report : reports) {
        safe_reports_count += is_safe_report(report);
        fixing_reports_count += is_fixing_report(report);
    }

    std::cout << "First star: " << safe_reports_count << std::endl;
    std::cout << "Second star: " << fixing_reports_count << std::endl;

    return 0;
}
