import numpy as np
import matplotlib.pyplot as plt
import struct

def read_pps():
	with open("pps.bin", "rb") as binary:
		data = [[[[0.0 for _ in range(17)] for _ in range(9)] for _ in range(15)] for _ in range(15)]
		for i in range(15):
			for j in range(15):
				for k in range(9):
					for l in range(17):
						binexp = binary.read(4)
						# if binexp != b'`f\x06\xc0': print(binexp, [i, j], struct.unpack('<f', binexp)[0])
						data[i][j][k][l] = struct.unpack('<f', binexp)[0]
		return data

def read_ppo():
	with open("ppo.bin", "rb") as binary:
		data = [[[[0.0 for _ in range(17)] for _ in range(17)] for _ in range(15)] for _ in range(15)]
		for i in range(15):
			for j in range(15):
				for k in range(17):
					for l in range(17):
						binexp = binary.read(4)
						data[i][j][k][l] = struct.unpack('<f', binexp)[0]
		return data

def feature(arr):
	lst = []
	for i in range(len(arr)):
		for j in range(len(arr[0])):
			lst.append([max(arr[i][j]), i, j])
	lst = sorted(lst)[::-1]
	for i in range(5):
		show(arr, lst[i][1], lst[i][2])

def show(arr, i, j):
	data = [[0.0 for _ in arr[0][0][0]] for _ in arr[0][0]]
	for k in range(len(arr[0][0])):
		for l in range(len(arr[0][0][0])):
			data[k][l] = arr[i][j][k][l]
	print(i, j)
	print(np.array(data))
	fig, ax = plt.subplots()
	ax.imshow(data, interpolation='nearest')
	numrows, numcols = len(arr[0][0]), len(arr[0][0][0])
	plt.show()

if __name__ == "__main__":
	PPs = read_pps()
	PPo = read_ppo()
	feature(PPs)
	# while 1:
	# 	i, j = map(int, input().split())
	# 	show(PPs, i, j)
