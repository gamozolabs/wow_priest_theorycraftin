set term wxt persist size 1440,900
set grid
set xlabel "Health"
set ylabel "Healing Until OOM (Rank 3 'Heal')"
set title "29s Twink Holy Undead Priest heal-until-OOM (no regen) vs health"
plot "bvph.txt" u 1:2 w l

