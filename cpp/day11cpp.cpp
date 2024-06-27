#include <algorithm>
#include <cstddef>
#include <cstdlib>
#include <cstring>
#include <cstdint>
#include <numeric>
#include <set>
#include <span>
#include <vector>

struct Galaxy {
    const size_t x;
    const size_t y;

    Galaxy(size_t x, size_t y) : x(x), y(y) {}

    size_t distance(const Galaxy &other) const {
        return std::abs((long)x - (long)other.x) + std::abs((long)y - (long)other.y);
    }
};

struct Universe {
    const std::vector<Galaxy> galaxies;
    const std::set<size_t> empty_columns;
    const std::set<size_t> empty_rows;
    size_t extra_space;

    Universe(const std::vector<Galaxy> galaxies, const std::set<size_t> empty_columns,
             const std::set<size_t> empty_rows, const size_t extra_space)
        : galaxies(galaxies), empty_columns(empty_columns),
          empty_rows(empty_rows), extra_space(extra_space) {}

    Galaxy move_by_expansion(const Galaxy &galaxy) const {
        size_t x = count_if(empty_columns.begin(), empty_columns.end(),
        [&](size_t column) {
            return column < galaxy.x;
        }) *
        extra_space +
        galaxy.x;
        ;
        size_t y = count_if(empty_rows.begin(), empty_rows.end(),
        [&](size_t row) {
            return row < galaxy.y;
        }) *
        extra_space +
        galaxy.y;
        return Galaxy(x, y);
    }

    size_t sum_of_all_distances_after_expansion() const {
        std::vector<Galaxy> expanded_galaxies;
        for (const auto &galaxy : galaxies) {
            expanded_galaxies.push_back(move_by_expansion(galaxy));
        }
        size_t sum = 0;
        for (auto first = expanded_galaxies.begin();
                first != expanded_galaxies.end(); ++first) {
            for (auto second = first + 1; second != expanded_galaxies.end();
                    ++second) {
                sum += first->distance(*second);
            }
        }
        return sum;
    }

    static Universe big_bang(const std::span<const uint8_t> data,
                             const size_t extra_space) {
        // we assume that all lines are the same length
        size_t line_length =
            (const uint8_t *)memchr(data.data(), '\n', data.size()) -
            data.data();
        std::vector<Galaxy> galaxies;
        std::set<size_t> empty_columns;
        std::set<size_t> empty_rows = {};

        // there are more elegant ways to do this in C++23,
        // but most compilers do not support it yet
        for (size_t i = 0; i < line_length; ++i) {
            empty_columns.insert(i);
        }
        for (size_t i = 0; i < (data.size() / (line_length + 1)); ++i) {
            empty_rows.insert(i);
        }

        auto start_offset = 0;
        auto p = memchr(data.data() + start_offset, '#', data.size() - start_offset);
        while (p != nullptr) {
            size_t offset = (const uint8_t *)p - data.data();
            size_t x = offset % (line_length + 1);
            size_t y = offset / (line_length + 1);
            galaxies.push_back(Galaxy(x, y));
            empty_columns.erase(x);
            empty_rows.erase(y);
            start_offset = offset + 1;
            p = memchr(data.data() + start_offset, '#', data.size() - start_offset);
        }

        return Universe(galaxies, empty_columns, empty_rows, extra_space);
    }
};

extern "C" {
    size_t part1_cpp(const uint8_t *input, size_t input_len) {
        auto span = std::span(input, input_len);
        auto universe = Universe::big_bang(span, 1);
        return universe.sum_of_all_distances_after_expansion();
    }
    size_t part2_cpp(const uint8_t *input, size_t input_len) {
        auto span = std::span(input, input_len);
        auto universe = Universe::big_bang(span, 999999);
        return universe.sum_of_all_distances_after_expansion();
    }
}