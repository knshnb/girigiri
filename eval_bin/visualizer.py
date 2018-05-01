import numpy as np
import matplotlib.pyplot as plt
import struct

def read_pps():
	with open("pps.bin", "rb") as binary:
		data = [[[[0.0] * 17] * 9] * 15] * 15
		for i in range(15):
			for j in range(15):
				for k in range(9):
					for l in range(17):
						data[i][j][k][l] = struct.unpack('<f', binary.read(4))[0]
	return data

def read_ppo():
	with open("ppo.bin", "rb") as binary:
		data = [[[[0.0] * 17] * 17] * 15] * 15
		for i in range(15):
			for j in range(15):
				for k in range(17):
					for l in range(17):
						data[i][j][k][l] = struct.unpack('<f', binary.read(4))[0]
	return data

def show(arr, i, j):
	data = [[0.0] * len(arr[0][0][0])] * len(arr[0][0])
	for k in range(len(arr[0][0])):
		for l in range(len(arr[0][0][0])):
			data[k][l] = arr[i][j][k][l]
	print(np.array(data))
	fig, ax = plt.subplots()
	ax.imshow(data, interpolation='nearest')
	numrows, numcols = len(arr[0][0]), len(arr[0][0][0])
	plt.show()

if __name__ == "__main__":
	PPs = read_pps()
	PPo = read_ppo()
	while 1:
		i, j = map(int, input().split())
		show(PPs, i, j)
