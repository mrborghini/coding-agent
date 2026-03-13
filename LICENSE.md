### **AIR-L: The AI Reciprocity License (v1.0)**

**Version 1.0, March 2026**

This license is a derivative work of the GNU GPL v2, modified to address the unique challenges of Artificial Intelligence. It is NOT the GNU GPL. It is a custom license designed to ensure that intelligence derived from this Program remains open to the public.

---

#### **TERMS AND CONDITIONS**

**0. Definitions**

"The Program" refers to any program, source code, documentation, or work placed under this license.

"The Model" refers to any neural network weights, parameters, or weight-sets created by training on, fine-tuning with, or optimizing against the Program, its source code, its documentation, or its generated outputs.

A "work based on the Program" means either the Program or any derivative work under copyright law. For the purposes of this License, a "work based on the Program" shall also include any "Computational Intelligence Work." This includes, but is not limited to, any Machine Learning Model, Large Language Model (LLM), or Neural Network weight-set - i.e. any instance of The Model.

**1. Distribution of Source Code**
You may copy and distribute verbatim copies of the Program's source code as you receive it, provided that you conspicuously publish on each copy an appropriate copyright notice and disclaimer of warranty; keep intact all notices that refer to this License; and give any other recipients of the Program a copy of this License along with the Program.

**2. Modifications and Derivative Works**
You may modify your copy or copies of the Program, provided that you meet all of the following conditions:

* **a)** You must cause the modified files to carry prominent notices stating that you changed the files and the date of any change.
* **b)** You must cause any work that you distribute or publish, that in whole or in part contains or is derived from the Program, to be licensed as a whole at no charge to all third parties under the terms of this License.
* **c)** If the modified program normally reads commands interactively when run, you must cause it, when started running, to print or display an announcement including an appropriate copyright notice and a notice that there is no warranty.
* **d)** If you use the Program as training data for an AI model, you must license that model's weights and the training pipeline under these same terms. You may not distribute a model trained on this code as a "closed-source" or "proprietary" product.
* **e) AI Reciprocity:** If you utilize the Program as part of a training process to create a Computational Intelligence Work, you must license the resulting model weights, training scripts, and data manifests as a whole at no charge to all third parties under the terms of this License.

**3. The "Source Code" for AI Models**
For an executable work, "complete source code" means the preferred form for making modifications. Under this license, if the work is a Computational Intelligence Work (The Model), "complete source code" is defined as:

* **The Model Weights:** The full, unquantized parameters required to run the model.
* **The Training Pipeline:** The scripts, hyperparameters, and code used to process the data and perform the training.
* **The Data Recipe:** A comprehensive manifest of the training data. If the data itself cannot be distributed due to third-party restrictions, you must provide a "reproduction guide" sufficient for a peer to recreate a substantially similar dataset.

**4. Termination**
Any attempt to train a model using this Program without complying with the reciprocity requirements of Section 2(d), Section 2(e), and Section 3 automatically terminates your rights under this License. Continued use of the Program or the resulting model constitutes copyright infringement.

**5. Acceptance**
By using this Program for the purpose of training an artificial intelligence, you indicate your acceptance of this License and all its terms and conditions regarding the reciprocity of model weights.

**6. No Warranty**
THE PROGRAM IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY ARISING FROM THE USE OF THE PROGRAM.

---

#### **How to Apply This License to Your Work**

To apply this license, attach the following notice to your program. It is safest to attach it to the start of each source file:

> This program is free software; you can redistribute it and/or modify it under the terms of the AIR-L (AI Reciprocity License) as found in this repository.
