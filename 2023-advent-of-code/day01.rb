["example01.txt", "input01.txt"].each do |file|
    File.readlines(file).each do |line|
        [line.chars, line.chars.reverse].each { |line|
            line.chars.find { |c| c.isDigit? }
        }
        .join
        .to_i
    end
end