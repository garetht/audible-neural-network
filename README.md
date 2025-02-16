# An Audible Neural Network

This is an implementation of a three-layer feedforward neural network from the ground up in Rust using only `ndarrays`. See `./src/network.rs`. PyTorch and other machine learning libraries were specifically avoided. The forward pass is done through tensor manipulation and the backwards pass is done through calculating hand-derived expressions. 

The object of this exercise is to produce data during the training of the network that controls musical parameters (distortion and a phase effect) to indicate the approach of the weights to a point that reduces the loss.
