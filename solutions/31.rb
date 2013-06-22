count = 0
200.step(0, -200) {|a|
    a.step(0, -100) {|b|
        b.step(0, -50){|c|
            c.step(0, -20){|d|
                d.step(0, -10){|e|
                    e.step(0, -5){|f| 
                        f.step(0, -2){|g|
                            count += 1}}}}}}}
p count