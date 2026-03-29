// HyzeScript/runtime/optim.hpp

#ifndef OPTIM_HPP
#define OPTIM_HPP

#include "Tensor.hpp"
#include <vector>

class SGD {
public:
    double lr;

    SGD(double learning_rate) : lr(learning_rate) {}

    // Very simplified: assume you pass a Tensor that needs updating
    void step(Tensor& param_grad) {
        for (double& g : param_grad.data) {
            g *= lr;
        }
    }
};

#endif // OPTIM_HPP
