<?php
require('library.php');
do_top(9, 'TensorFlow');
?>

<p>
To make a
<a href="https://en.wikipedia.org/wiki/Convolutional_neural_network">
convolutional neural network</a>
using
<a href="https://en.wikipedia.org/wiki/Keras">Keras</a>
the code is easy and is just:
  </p>

<pre><code class="codeClass">
#!/usr/bin/env python
  
import tensorflow as tf
from tensorflow import keras
from tensorflow.keras import layers
from keras.preprocessing.image import ImageDataGenerator

model = keras.Sequential(
  [
    layers.Conv2D(32, (3, 3), input_shape=(150, 150, 3)),# input shape 255x255 pixels, 3 channels for RGB
    layers.Activation('relu'),
    layers.MaxPooling2D(pool_size=(2, 2)),
    
    layers.Conv2D(32, (3, 3)),
    layers.Activation('relu'),
    layers.MaxPooling2D(pool_size=(2, 2)),

    layers.Conv2D(32, (3, 3)),
    layers.Activation('relu'),
    layers.MaxPooling2D(pool_size=(2, 2)),

    
    layers.Flatten(),
    layers.Dense(64),
    layers.Activation('relu'),
    layers.Dropout(0.5),
    layers.Dense(12, activation="softmax") #the kaggle data has 12 seedling categories
  ]
)

model.compile(loss='categorical_crossentropy', #because seeds aren't binary, have 12 categories
              optimizer='rmsprop',
              metrics=['accuracy'])

model.summary()

</code></pre>
  <p>I chose the shape of the neural network arbitrarily but anyone can change the shape and kernel sizes of the convolutional layers to whatever they want. At the end, it is flattened and compressed into 12 outputs because the <a href="https://www.kaggle.com/c/plant-seedlings-classification/data">data set I am going to be testing on</a> has 12 categories of seedlings.
The twelve <a href="https://en.wikipedia.org/wiki/Softmax_function">softmax</a> outputs represent the probability of each seedling classification and sum to 1.0.
</p>
  
<br>

<p>To run the model on the data I put the training data in ./data/train and the validation data in ./data/test. Keras has an easy to use tool called the image data generator. You can give it a folder of training data and each sub-folder will be a category of that data. For example ./data/train/Charlock is a category and ./data/train/Cleavers is another category. There are 12 total subfolders and thus 12 categories of plants.</p>

<br>
<img width="700" height="478" src="imgs/seedlings_examples.jpg" />
<br>
  <p>The really cool part about the image data generator is that it can automatically shift around, rotate, and flip the image to force the AI to be more robust and prevent overfitting. Here you can see it randomly rotates it between 0 and 180 degrees, shifts the image up and down randomly 20%, sheers and zooms randomly 20%, and randomly flips it horizontally and vertically.</p>
  
<pre><code class="codeClass">
batch_size = 16

train_datagen = ImageDataGenerator(
  rotation_range=180,
  width_shift_range=0.2,
  height_shift_range=0.2,
  rescale=1./255,
  shear_range=0.2,
  zoom_range=0.2,
  horizontal_flip=True,
  vertical_flip=True,
  fill_mode='nearest')

test_datagen = ImageDataGenerator(rescale=1./255)

train_generator = train_datagen.flow_from_directory(
  'data/train',
  target_size=(150, 150),#must be same as input shape
  batch_size=batch_size,
  class_mode='categorical') #same as compile loss

# this is a similar generator, for validation data
validation_generator = test_datagen.flow_from_directory(
  'data/test',
  target_size=(150, 150),
  batch_size=batch_size,
  class_mode='categorical')

model.fit_generator(
  train_generator,
  steps_per_epoch=2000,
  epochs=50,
  validation_data = validation_generator,
  validation_steps=800
)

mode.save_weights('output.h5') #save to be loaded later
</code></pre>

  <p>After running, it will save the outout to the .h5 file which can then be loaded again to either use or train more</p>
<hr/>
  
<?php   
do_footer();
?>
