from data_access_layer.data_access_layer import read_rgb_image


def main():
    height = 480
    width = 853
    data = read_rgb_image("./assets/test.jpg", width, height)
    print(f"\n\nThe image has {len(data)} pixels\n\n")


if __name__ == "__main__":
    main()
