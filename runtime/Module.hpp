// HyzeScript/runtime/Module.hpp

#ifndef MODULE_HPP
#define MODULE_HPP

#include "Tensor.hpp"
#include <memory>

class Module {
public:
    virtual ~Module() = default;
    virtual Tensor forward(const Tensor& x) const = 0;
    virtual std::unique_ptr<Module> clone() const = 0;
};

#endif // MODULE_HPP
