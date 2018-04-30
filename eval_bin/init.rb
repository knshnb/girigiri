s = ""
(14*14).times {
	s += "\x00\x00\x00\x00"
}

t = ""
(9*9).times {
	t += s
}

File.binwrite("pps.bin", t)

t = ""
(17*17).times {
	t += s
}

File.binwrite("ppo.bin", t)
