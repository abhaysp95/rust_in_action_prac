set key off
set rmargin 5
set grid ytics noxtics nocbtics back
set border 3 back lw 2 lc rgbcolor "#222222"

set xlabel "Allocation size (bytes)"
set logscale x 2
set xtics nomirror out
set xrange [0 to 100000]

set ylabel "Allocation duration (ns)"
set logscale y
set ytics nomirror out
set yrange [0 to 10000]

plot "alloc.tsv" with points \
	pointtype 6 \
	pointsize 1.25 \
	linecolor rgbcolor "#22dd3131"
