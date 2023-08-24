#! /usr/bin/Rscript

#! /usr/bin/Rscript

f<-file("stdin")
open(f)
record<-c()
while(length(line<-readLines(f,n=1))>0){
  #write(line,stderr())
  record<-c(record,line)  
}


numbers<-record[1:length(record)-1]
numbers_<-c()


print(getwd())


coordinates <- strsplit(record, " ")

# Extract x, y, and time values
x <- as.numeric(sapply(coordinates, "[[", 1))
y <- as.numeric(sapply(coordinates, "[[", 2))
time <- as.numeric(sapply(coordinates, "[[", 3))

# Plot heatmap
library(ggplot2)
library(pandoc)

# Extract x, y coordinates
coordinates <- strsplit(record, " ")

# Extract x, y, and time values
x <- as.numeric(sapply(coordinates, "[[", 1))
y <- as.numeric(sapply(coordinates, "[[", 2))
time <- as.numeric(sapply(coordinates, "[[", 3))

# Plot heatmap
library(ggplot2)
library(plotly)

data <- data.frame(x = x, y = y)
heatmap_plot <- ggplot(data, aes(x, y)) +
  geom_bin2d(bins = 20) +  # Adjust bins as needed
  scale_fill_viridis_c() +  # You can choose other color scales too
  labs(title = "Heatmap of Coordinates")

# Convert ggplot to ggplotly
heatmap_interactive <- ggplotly(heatmap_plot)

# Save as HTML using pandoc
# htmlwidgets::saveWidget(heatmap_interactive, "heatmap_output.html", selfcontained = TRUE)
print("Heatmap generated successfully!")




#Finding all files that contain this name are as follows
df<-read.csv("output.csv",header = FALSE)

library("ggplot2")
library("plotly")
library("breakDown")
library(ggdark)
library(pracma)
library(comprehenr)
library(ggridges)
library(tidyverse)
library(ggplot2)
library(plotly)
library(thematic)
library(extrafont)
library(pandoc)
#library(pandoc)
#Get dem custom fonts
font_import()
loadfonts(device = "win")
# actual_pars<-as.data.frame(actual_pars)



colnames(data) <- c(
  "HitPct1", "TotalSamples1", "HitSamples1",
  "HitPct2", "TotalSamples2", "HitSamples2",
  "HitPct3", "TotalSamples3", "HitSamples3",
  "HitPct4", "TotalSamples4", "HitSamples4"
)

# Scatter plot for the first 2 sets of data
# Define custom theme colors
thematic_on(bg = "#FCE9D7", fg = "orange", accent = "purple")

# Scatter plot for the first 2 sets of data
scatter_plot_1 <- ggplot(data, aes(x = HitPct1, y = HitPct2)) +
  geom_point() +
  labs(title = "Scatter Plot: First 2 Sets") +
  theme_bw() +  # Applying the custom theme
  theme(
    legend.background = element_rect(
      fill = "#FCE9D7",
      colour = "black",
      size = 1
    )
  )


# Scatter plot for the last 2 sets of data
# Scatter plot for the last 2 sets of data
scatter_plot_2 <- ggplot(data, aes(x = HitPct3, y = HitPct4)) +
  geom_point() +
  labs(title = "Scatter Plot: Last 2 Sets") +
  theme_bw() +  # Applying the custom theme
  theme(
    legend.background = element_rect(
      fill = "#FCE9D7",
      colour = "black",
      size = 1
    )
  )

# Convert ggplot to ggplotly
scatter_interactive_1 <- ggplotly(scatter_plot_1)
scatter_interactive_2 <- ggplotly(scatter_plot_2)
htmlwidgets::saveWidget(scatter_interactive_1, "scatter_plot_1.html", selfcontained = TRUE)
htmlwidgets::saveWidget(scatter_interactive_2, "scatter_plot_2.html", selfcontained = TRUE)