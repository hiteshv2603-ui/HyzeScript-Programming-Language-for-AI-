// HyzeScript/runtime/nn.hpp

#ifndef NN_HPP
#define NN_HPP

#include "Tensor.hpp"
#include <random>

class Linear {
public:
    // weight: in_features * out_features
    // bias: out_features
    Tensor weight;
    Tensor bias;

    Linear(size_t in_features, size_t out_features)
        : weight({0.0}, {in_features, out_features}),
          bias({0.0}, {out_features}) {
        std::minstd_rand0 rng(123);
        std::uniform_real_distribution<double> dist(-0.5, 0.5);

        for (double& w : weight.data) {
            w = dist(rng);
        }
        for (double& b : bias.data) {
            b = dist(rng);
        }
    }

    // x: batch * in_features
    // returns: batch * out_features
    Tensor forward(const Tensor& x) const {
        if (x.shape.size() != 2) {
            throw std::runtime_error("Linear::forward expects 2D input (batch, in_features)");
        }
        if (x.shape[1] != weight.shape[0]) {
            throw std::runtime_error("Linear::forward: input features do not match");
        }

        // out = x @ weight + bias
        Tensor out = x.matmul(weight);

        // broadcast bias (add to each row)
        for (size_t i = 0; i < out.shape[0]; ++i) {
            for (size_t j = 0; j < out.shape[1]; ++j) {
                out(i * out.shape[1] + j) += bias(j);
            }
        }

        return out;
    }
};

class ReLU {
public:
    Tensor forward(const Tensor& x) const {
        Tensor out({0.0}, x.shape);
        for (size_t i = 0; i < x.data.size(); ++i) {
            out.data[i] = x.data[i] > 0.0 ? x.data[i] : 0.0;
        }
        return out;
    }
};

class Sequential {
public:
    std::vector<std::unique_ptr<Module>> modules;

    void add(const std::unique_ptr<Module>& m) {
        modules.push_back(m->clone());
    }

    Tensor forward(const Tensor& x) const {
        Tensor out = x;
        for (const auto& m : modules) {
            out = m->forward(out);
        }
        return out;
    }

    virtual ~Module() = default;

    virtual Tensor forward(const Tensor& x) const = 0;
    virtual std::unique_ptr<Module> clone() const = 0;
};

#endif // NN_HPP
