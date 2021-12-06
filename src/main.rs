fn main() {
    // let input = _get_test_input();
    let input = _get_input();


    // ----------- Parse Input ----------- 

    let lines = input.trim().split("\n")
        .map(|s| {
            let mut split = s.split(" ");
            let start = split.next().unwrap().split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            split.next();
            let end = split.next().unwrap().split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            return ((start[0], start[1]), (end[0], end[1]));
        }).collect::<Vec<((i32, i32), (i32, i32))>>();

    // ----------- Solve ----------- 

    let mut grid = [[0; 1000]; 1000].to_vec();
    let mut grid_p1 = [[0; 1000]; 1000].to_vec();

    for line in &lines {
        let mut p = line.0;
        grid[p.1 as usize][p.0 as usize] += 1;
        if line.0.0 == line.1.0 || line.0.1 == line.1.1 {
            grid_p1[p.1 as usize][p.0 as usize] += 1;
        }
        loop {
            p.0 += if line.0.0 < line.1.0 { 1 } else if line.0.0 > line.1.0 { -1 } else { 0 };
            p.1 += if line.0.1 < line.1.1 { 1 } else if line.0.1 > line.1.1 { -1 } else { 0 };
            grid[p.1 as usize][p.0 as usize] += 1;
            if line.0.0 == line.1.0 || line.0.1 == line.1.1 {
                grid_p1[p.1 as usize][p.0 as usize] += 1;
            }
            if p.0 == line.1.0 && p.1 == line.1.1 {
                break;
            }
        }
    }
    let mut count_p1 = 0;
    for row in grid_p1 {
        for cell in row {
            if cell > 1 {
                count_p1 += 1;
            }
        }
    }

    let mut count_p2 = 0;
    for row in grid {
        for cell in row {
            if cell > 1 {
                count_p2 += 1;
            }
        }
    }
    

    // ----------- Print ----------- 

    println!("Part 1: {:?}", count_p1);
    println!("Part 2: {:?}", count_p2);
}

fn _get_test_input() -> String {
    return "

0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2

"
    .to_string();
}

fn _get_input() -> String {
    return "

911,808 -> 324,221
161,890 -> 808,243
174,59 -> 174,760
983,983 -> 10,10
321,12 -> 870,12
66,936 -> 941,61
670,141 -> 670,550
783,935 -> 496,648
973,651 -> 635,989
535,47 -> 535,154
355,183 -> 754,582
172,111 -> 892,111
353,66 -> 907,620
741,960 -> 741,805
113,895 -> 946,895
777,280 -> 563,280
679,815 -> 626,815
651,848 -> 651,673
205,834 -> 205,599
895,118 -> 82,931
685,303 -> 93,895
973,38 -> 62,949
867,23 -> 867,300
784,947 -> 784,47
96,168 -> 755,827
909,321 -> 909,716
59,881 -> 692,881
964,19 -> 69,914
752,869 -> 67,184
974,877 -> 138,41
432,389 -> 137,684
458,822 -> 458,402
818,852 -> 308,342
882,484 -> 441,925
82,959 -> 976,65
117,487 -> 117,429
214,673 -> 429,673
72,955 -> 72,829
587,109 -> 587,368
576,17 -> 576,872
685,102 -> 685,905
563,394 -> 716,394
966,145 -> 150,961
555,582 -> 555,385
453,31 -> 453,207
639,815 -> 547,723
431,869 -> 431,811
646,938 -> 599,938
215,513 -> 900,513
809,82 -> 798,82
768,344 -> 244,868
39,962 -> 39,601
675,186 -> 61,186
861,967 -> 28,967
860,550 -> 538,550
283,740 -> 571,740
72,297 -> 72,645
727,801 -> 727,526
799,519 -> 799,497
915,24 -> 174,765
795,943 -> 136,943
518,971 -> 599,971
594,676 -> 594,461
850,799 -> 363,799
958,575 -> 958,231
752,576 -> 398,576
891,433 -> 398,433
524,126 -> 397,126
10,890 -> 796,104
57,228 -> 168,228
168,521 -> 338,691
230,83 -> 777,83
865,677 -> 640,452
866,821 -> 825,821
17,143 -> 17,596
113,916 -> 113,601
268,187 -> 551,470
794,167 -> 220,167
459,17 -> 459,931
211,31 -> 526,31
680,57 -> 756,57
926,190 -> 926,800
85,284 -> 63,284
44,988 -> 44,701
110,941 -> 176,941
480,163 -> 480,112
574,538 -> 574,371
584,473 -> 69,473
303,621 -> 303,380
762,652 -> 762,89
286,195 -> 276,185
957,87 -> 217,827
561,858 -> 561,437
384,55 -> 81,55
19,977 -> 981,15
454,747 -> 938,263
425,836 -> 425,617
860,135 -> 775,50
633,131 -> 633,651
904,912 -> 242,250
880,177 -> 480,577
470,162 -> 964,656
585,376 -> 585,470
696,760 -> 594,862
534,225 -> 534,717
258,816 -> 258,847
990,244 -> 990,93
463,462 -> 463,533
434,928 -> 537,825
813,734 -> 533,734
498,673 -> 395,673
564,312 -> 55,312
280,550 -> 939,550
591,247 -> 396,52
127,516 -> 127,235
850,425 -> 552,127
894,428 -> 894,598
366,960 -> 592,960
579,488 -> 170,488
775,92 -> 775,586
49,909 -> 930,28
856,113 -> 284,685
263,175 -> 120,175
332,592 -> 276,592
920,157 -> 141,157
349,776 -> 316,776
187,863 -> 279,863
218,872 -> 83,872
465,430 -> 410,430
710,218 -> 857,218
797,314 -> 184,314
387,327 -> 49,665
950,812 -> 205,67
803,133 -> 803,682
125,972 -> 545,552
353,901 -> 840,414
936,843 -> 202,109
11,904 -> 856,59
725,757 -> 954,986
227,697 -> 345,697
187,520 -> 187,441
860,262 -> 135,987
700,95 -> 976,371
86,946 -> 869,163
898,806 -> 461,806
717,796 -> 717,195
882,127 -> 835,127
133,48 -> 133,191
521,51 -> 521,927
384,806 -> 957,233
570,139 -> 570,842
949,819 -> 949,350
592,230 -> 283,230
315,856 -> 741,856
870,674 -> 549,353
857,306 -> 857,889
428,217 -> 267,217
47,93 -> 898,944
636,238 -> 665,238
202,910 -> 202,737
246,432 -> 617,803
985,24 -> 48,961
965,876 -> 956,867
618,650 -> 810,458
292,356 -> 575,356
394,585 -> 910,585
137,453 -> 137,178
509,737 -> 509,665
193,350 -> 531,688
805,219 -> 107,219
975,506 -> 907,506
435,303 -> 435,380
344,83 -> 344,224
47,66 -> 47,115
570,516 -> 857,516
162,91 -> 926,91
759,417 -> 759,460
445,942 -> 445,699
421,340 -> 421,743
590,590 -> 434,434
453,38 -> 453,327
865,134 -> 865,773
842,609 -> 18,609
662,282 -> 62,882
489,32 -> 344,32
135,496 -> 93,454
552,211 -> 421,211
620,678 -> 642,678
782,158 -> 585,355
733,509 -> 733,574
932,383 -> 369,946
843,705 -> 843,725
747,414 -> 676,343
294,218 -> 962,886
844,175 -> 844,420
255,489 -> 531,213
555,532 -> 821,532
533,15 -> 533,161
631,778 -> 631,401
75,282 -> 468,282
903,838 -> 903,957
46,293 -> 543,790
30,834 -> 30,948
591,720 -> 591,965
624,36 -> 339,36
425,323 -> 425,442
234,939 -> 234,963
482,912 -> 968,912
228,614 -> 189,614
969,472 -> 969,692
871,494 -> 871,172
101,624 -> 848,624
424,918 -> 69,563
929,671 -> 93,671
81,187 -> 707,813
348,923 -> 348,924
921,524 -> 921,828
678,454 -> 678,364
904,227 -> 904,596
163,344 -> 609,790
206,180 -> 206,59
145,519 -> 145,717
317,679 -> 317,417
503,724 -> 221,724
353,448 -> 413,448
363,643 -> 837,643
594,54 -> 359,54
866,117 -> 45,938
939,210 -> 284,865
410,556 -> 410,801
905,111 -> 673,111
983,167 -> 574,167
595,758 -> 97,758
785,10 -> 437,10
517,414 -> 517,734
691,567 -> 186,62
842,51 -> 31,862
36,199 -> 282,199
864,758 -> 864,610
639,918 -> 951,918
245,516 -> 245,474
951,203 -> 557,203
176,728 -> 176,171
322,217 -> 387,217
149,208 -> 836,895
661,298 -> 609,298
46,47 -> 981,982
769,45 -> 769,610
988,932 -> 988,459
901,97 -> 901,908
195,395 -> 121,395
197,403 -> 327,533
159,456 -> 857,456
480,981 -> 881,580
86,958 -> 962,82
375,198 -> 763,198
950,381 -> 341,381
504,679 -> 504,598
756,659 -> 680,583
146,328 -> 886,328
930,412 -> 492,850
954,54 -> 954,940
790,498 -> 790,305
83,270 -> 83,242
939,268 -> 939,563
423,756 -> 916,263
583,756 -> 583,34
957,639 -> 614,639
484,523 -> 521,560
497,809 -> 497,419
76,17 -> 979,920
49,39 -> 943,933
110,289 -> 110,247
874,868 -> 874,172
576,127 -> 53,650
871,879 -> 12,20
436,711 -> 592,711
132,285 -> 225,285
245,147 -> 514,147
158,882 -> 956,84
21,984 -> 937,68
42,275 -> 219,275
877,143 -> 889,143
593,841 -> 508,756
414,289 -> 132,289
687,655 -> 767,655
453,981 -> 459,987
635,433 -> 635,324
671,347 -> 170,848
412,579 -> 915,579
269,677 -> 269,596
587,121 -> 367,341
153,883 -> 153,709
524,580 -> 508,580
541,232 -> 651,232
93,948 -> 284,757
168,745 -> 872,41
831,657 -> 925,563
908,389 -> 442,389
462,445 -> 234,445
735,493 -> 895,493
274,624 -> 296,646
153,130 -> 153,160
466,214 -> 466,769
474,499 -> 686,711
540,428 -> 788,676
858,215 -> 959,215
788,91 -> 788,410
552,505 -> 988,505
978,312 -> 978,202
108,321 -> 616,829
903,359 -> 903,770
480,331 -> 480,769
503,842 -> 34,842
613,732 -> 323,442
767,949 -> 654,949
514,589 -> 386,589
38,554 -> 284,308
689,268 -> 689,711
860,66 -> 190,736
253,865 -> 622,865
87,658 -> 698,47
506,892 -> 829,569
680,910 -> 594,824
824,603 -> 958,603
576,802 -> 562,802
67,27 -> 67,489
969,461 -> 517,913
674,763 -> 674,226
223,955 -> 218,955
147,540 -> 569,962
455,703 -> 596,703
746,899 -> 746,403
516,476 -> 756,476
897,674 -> 373,150
120,395 -> 120,49
722,443 -> 722,244
724,924 -> 724,39
809,930 -> 109,930
822,816 -> 874,816
796,539 -> 895,539
340,88 -> 560,88
223,158 -> 593,158
779,977 -> 856,900
617,461 -> 973,817
515,62 -> 515,140
12,586 -> 724,586
870,50 -> 391,50
308,123 -> 308,696
119,164 -> 863,908
755,599 -> 448,599
129,526 -> 633,526
478,668 -> 102,668
237,637 -> 237,743
270,102 -> 72,300
115,470 -> 115,427
948,233 -> 948,731
983,135 -> 468,650
748,439 -> 748,642
62,862 -> 352,572
765,901 -> 660,901
917,807 -> 917,587
55,81 -> 116,81
954,972 -> 102,120
340,503 -> 294,549
970,661 -> 522,213
618,92 -> 618,247
688,965 -> 965,965
94,241 -> 94,292
15,132 -> 15,492
979,927 -> 488,927
509,26 -> 984,26
840,530 -> 840,95
55,956 -> 849,162
297,297 -> 297,472
338,780 -> 369,780
487,292 -> 37,292
122,117 -> 206,201
66,807 -> 564,309
643,242 -> 906,242
909,833 -> 909,441
129,128 -> 818,817
406,42 -> 406,297
53,20 -> 967,934
235,285 -> 601,285
275,625 -> 275,539
199,732 -> 430,963
639,187 -> 639,265
549,740 -> 549,824
603,140 -> 603,748
35,455 -> 176,455
888,611 -> 888,271
134,154 -> 484,154
694,820 -> 694,814
535,584 -> 187,932
642,510 -> 642,249
191,886 -> 268,886
918,353 -> 881,390
977,13 -> 343,13
380,243 -> 271,134
410,758 -> 410,670
613,551 -> 519,645
963,84 -> 124,923
702,252 -> 821,252
405,237 -> 405,22
21,139 -> 21,510
548,499 -> 132,499
196,104 -> 196,680
739,145 -> 476,145
751,746 -> 91,746
975,628 -> 975,847
935,520 -> 935,450
803,372 -> 803,393
872,77 -> 872,373
339,130 -> 339,103
226,886 -> 226,45
794,647 -> 794,257
90,922 -> 889,123
615,971 -> 615,574
26,278 -> 26,719
838,88 -> 806,88
263,691 -> 804,150
309,721 -> 910,721
510,496 -> 960,946
195,236 -> 46,236
610,143 -> 610,610
891,412 -> 891,268
714,21 -> 156,579
320,935 -> 320,96
240,782 -> 449,782
754,472 -> 48,472
105,481 -> 529,57
451,301 -> 451,965
796,638 -> 796,185
908,553 -> 771,553
98,543 -> 490,935
481,159 -> 762,159
593,527 -> 419,353
86,391 -> 216,521
260,716 -> 42,716
734,538 -> 375,179
24,974 -> 975,23
402,466 -> 787,851
344,409 -> 262,327
803,443 -> 685,443
986,152 -> 249,152
125,738 -> 90,773
184,772 -> 184,746
729,829 -> 729,340
226,527 -> 226,375
936,231 -> 222,945
254,333 -> 254,167
451,234 -> 451,273
915,790 -> 568,443
869,794 -> 504,429
11,878 -> 836,53
821,231 -> 522,530
285,419 -> 732,866
191,272 -> 191,679
41,651 -> 225,651
30,13 -> 879,862
980,488 -> 20,488
27,187 -> 27,348
53,238 -> 53,514
778,306 -> 379,705
425,399 -> 425,60
162,859 -> 57,859
652,926 -> 652,589
962,489 -> 555,896
197,378 -> 436,617
310,190 -> 310,760
678,20 -> 678,713
390,653 -> 985,58
938,351 -> 656,69
881,39 -> 18,902
313,681 -> 323,681
910,907 -> 288,907
739,977 -> 739,132
856,479 -> 154,479
893,785 -> 761,785
405,247 -> 405,901
58,933 -> 808,183
643,156 -> 676,189
149,773 -> 357,773
479,518 -> 434,518
389,518 -> 556,685
858,449 -> 533,774
503,133 -> 409,133
340,315 -> 219,194
183,701 -> 183,242
810,151 -> 195,151
446,686 -> 446,912
968,482 -> 49,482
203,20 -> 203,667
493,516 -> 647,516
900,91 -> 634,91
660,554 -> 119,13
964,864 -> 964,919
871,293 -> 344,293
895,258 -> 972,258

"
    .to_string();
}
