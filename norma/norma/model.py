import os

# os.environ["TF_CPP_MIN_LOG_LEVEL"] = "3"

from keras import layers
import tensorflow as tf


def build_model():
    inputs = tf.keras.Input(shape=(131,))

    board = tf.keras.layers.Reshape((5, 5, 3))(inputs[:, :75])  # type: ignore
    move = tf.keras.layers.Reshape((5, 5, 2))(inputs[:, 75:125])  # type: ignore
    scalar = tf.keras.layers.Reshape((6,))(inputs[:, 125:])  # type: ignore

    conv_board = layers.Conv2D(64, (3, 3))(board)
    conv_board = layers.BatchNormalization(momentum=0.9)(conv_board)
    conv_board = layers.Activation("leaky_relu")(conv_board)

    conv_board = layers.Conv2D(64, (2, 2))(conv_board)
    conv_board = layers.BatchNormalization(momentum=0.9)(conv_board)
    conv_board = layers.Activation("leaky_relu")(conv_board)

    conv_board = layers.Flatten()(conv_board)


    conv_move = layers.Conv2D(64, (3, 3), activation="leaky_relu")(move)
    conv_board = layers.BatchNormalization(momentum=0.9)(conv_board)
    conv_board = layers.Activation("leaky_relu")(conv_board)

    conv_board = layers.BatchNormalization(momentum=0.9)(conv_board)
    conv_board = layers.Activation("leaky_relu")(conv_board)
    conv_move = layers.Flatten()(conv_move)

    concat = layers.concatenate([conv_board, conv_move, scalar])

    fc1 = layers.Dense(128, activation="leaky_relu")(concat)
    fc2 = layers.Dense(64, activation="leaky_relu")(fc1)
    output = layers.Dense(1, activation="linear")(fc2)

    return tf.keras.Model(inputs=inputs, outputs=output)


tiger_model = build_model()

tiger_model.compile(
    optimizer=tf.keras.optimizers.Adam(learning_rate=0.001),
    loss="mean_squared_error",
)


print(f"\n-----------------------------------")
print(f"Tiger Model:")
print(f"-----------------------------------")
tiger_model.summary()


goat_model = build_model()

goat_model.compile(
    optimizer=tf.keras.optimizers.Adam(learning_rate=0.001),
    loss="mean_squared_error",
)

print(f"\n-----------------------------------")
print(f"Goat Model:")
print(f"-----------------------------------")
goat_model.summary()


def load_model(name="magma"):
    if os.path.exists(f"weights/{name}/tiger.index"):
        tiger_model.load_weights(f"weights/{name}/tiger")
    else:
        tiger_model.save_weights(f"weights/{name}/tiger")

    if os.path.exists(f"weights/{name}/goat.index"):
        goat_model.load_weights(f"weights/{name}/goat")
    else:
        tiger_model.save_weights(f"weights/{name}/goat")
