// HyzeScript/runtime/Tensor.hpp

#ifndef TENSOR_HPP
#define TENSOR_HPP

#include <vector>
#include <cstddef>
#include <initializer_list>
#include <iostream>

class Tensor {
public:
    std::vector<double> data;
    std::vector<size_t> shape;

    // Constructor from 1D data and given shape
    Tensor(const std::initializer_list<double>& init_data,
           const std::vector<size_t>& dims)
        : data(init_data.begin(), init_data.end()), shape(dims) {
        if (!data_ok()) {
            throw std::runtime_error("data size does not match shape");
        }
    }

    // Default 0D tensor
    Tensor() : data({0.0}), shape({}) {}

    // Check if element count matches shape
    bool data_ok() const {
        size_t expected = 1;
        for (auto d : shape) expected *= d;
        return expected == data.size();
    }

    // Accessor: index into 1D data (row‑major)
    double& operator()(size_t i) { return data[i]; }
    const double& operator()(size_t i) const { return data[i]; }

    // Simple print
    void print() const {
        std::cout << "Tensor<";
        for (size_t i = 0; i < shape.size(); ++i) {
            std::cout << shape[i];
            if (i + 1 < shape.size()) std::cout << ",";
        }
        std::cout << ">(";
        for (size_t i = 0; i < data.size(); ++i) {
            std::cout << data[i];
            if (i + 1 < data.size()) std::cout << ",";
        }
        std::cout << ")\n";
    }

    // Simple elementwise add
    Tensor add(const Tensor& other) const {
        if (shape != other.shape) {
            throw std::runtime_error("shape mismatch in add");
        }
        Tensor out({0.0}, shape);
        for (size_t i = 0; i < data.size(); ++i) {
            out.data[i] = data[i] + other.data[i];
        }
        return out;
    }

    // Simple elementwise multiply
    Tensor mul(const Tensor& other) const {
        if (shape != other.shape) {
            throw std::runtime_error("shape mismatch in mul");
        }
        Tensor out({0.0}, shape);
        for (size_t i = 0; i < data.size(); ++i) {
            out.data[i] = data[i] * other.data[i];
        }
        return out;
    }

    // Matrix multiply for 2D tensors (very minimal, assumes 2D)
    Tensor matmul(const Tensor& other) const {
        if (shape.size() != 2 || other.shape.size() != 2) {
            throw std::runtime_error("matmul only implemented for 2D tensors");
        }
        size_t m = shape[0];
        size_t k = shape[1];
        size_t n = other.shape[1];
        if (k != other.shape[0]) {
            throw std::runtime_error("matmul shape mismatch");
        }

        std::vector<size_t> out_shape = {m, n};
        Tensor out({0.0}, out_shape);

        for (size_t i = 0; i < m; ++i) {
            for (size_t j = 0; j < n; ++j) {
                double sum = 0.0;
                for (size_t kk = 0; kk < k; ++kk) {
                    sum += (*this)(i * k + kk) * other(kk * n + j);
                }
                out(i * n + j) = sum;
            }
        }

        return out;
    }
};

#endif // TENSOR_HPP
